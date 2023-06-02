use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;
use serde::de::DeserializeOwned;

use super::client;
use super::value_bound_query::ValueBoundQuery;
use crate::models::news::latest::{LatestNewsPacket};
use crate::models::news::news::{NewsPacket};

use crate::models::packet::Packet;
use crate::models::streams::league_stream::LeagueStream;
use crate::models::streams::stream::{StreamPacket};
use crate::models::users::lists::league::{LeaguePacket};
use crate::models::users::lists::league_full::{LeagueFullPacket};
use crate::models::users::lists::xp::{XpPacket};
use crate::models::users::user_info::{UserInfoPacket};
use crate::models::users::user_records::{UserRecordsPacket};
use crate::models::users::user_search::{UserSearchPacket};

pub struct CachedClient {
    user_info_cache: Cache<String, Arc<UserInfoPacket>>,
    user_records_cache: Cache<String, Arc<UserRecordsPacket>>,
    user_search_cache: Cache<String, Arc<UserSearchPacket>>,
    full_league_leaderboard_cache: Cache<Option<String>, Arc<LeagueFullPacket>>,
    xp_leaderboard_cache: Cache<ValueBoundQuery, Arc<XpPacket>>,
    league_leaderboard_cache: Cache<ValueBoundQuery, Arc<LeaguePacket>>,
    stream_cache: Cache<String, Arc<StreamPacket>>,
    league_stream_cache: Cache<String, Arc<Packet<LeagueStream>>>,
    news_cache: Cache<Option<i64>, Arc<NewsPacket>>,
    latest_news_cache: Cache<(String, Option<i64>), Arc<LatestNewsPacket>>,
}

impl Default for CachedClient {
    fn default() -> Self {
        Self { 
            user_info_cache: Cache::builder().time_to_live(Duration::from_secs(300)).build(),
            user_records_cache: Cache::builder().time_to_live(Duration::from_secs(900)).build(),
            user_search_cache: Cache::builder().time_to_live(Duration::from_secs(300)).build(),
            league_leaderboard_cache: Cache::builder().time_to_live(Duration::from_secs(600)).build(),
            xp_leaderboard_cache: Cache::builder().time_to_live(Duration::from_secs(600)).build(),
            full_league_leaderboard_cache: Cache::builder().time_to_live(Duration::from_secs(3600)).build(),
            stream_cache: Cache::builder().time_to_live(Duration::from_secs(60)).build(),
            news_cache: Cache::builder().time_to_live(Duration::from_secs(60)).build(),
            latest_news_cache: Cache::builder().time_to_live(Duration::from_secs(60)).build(),
            league_stream_cache: Cache::builder().time_to_live(Duration::from_secs(60)).build()
        }
    }
}

use client::TETRIO_API_URL;

impl CachedClient {
    async fn make_tetrio_api_request<T: DeserializeOwned>(route: String) -> anyhow::Result<T> {
        let url = format!("{TETRIO_API_URL}{route}");
    
        Ok(reqwest::get(url).await?.json::<T>().await?)
    }
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_user_info("promooooooo").await.unwrap(); // Returns an Arc
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let user_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(&user_data.user.username);
    /// # });
    /// ```
    /// Invalid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_user_info("KAZEOIJAIZDHIQSUDH").await.unwrap();
    /// 
    /// assert!(!packet.success && packet.data.is_none() && packet.error.is_some());
    /// 
    /// let error = packet.error.as_ref().unwrap();
    /// 
    /// dbg!(&error);
    /// # });
    /// ```
    pub async fn fetch_user_info(&self, user: &str) -> anyhow::Result<Arc<UserInfoPacket>> {
        if let Some(data) = self.user_info_cache.get(user) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request::<UserInfoPacket>(format!("users/{user}")).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_info_cache.insert(user.to_owned(), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_user_records("takathedinosaur").await.unwrap(); // Returns an Arc
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let user_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(&user_data.records.blitz);
    /// # });
    /// ```
    /// Invalid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_user_records("KAZEOIJAIZDHIQSUDH").await.unwrap();
    /// 
    /// assert!(!packet.success && packet.data.is_none() && packet.error.is_some());
    /// 
    /// let error = packet.error.as_ref().unwrap();
    /// 
    /// dbg!(&error);
    /// # });
    /// ```
    pub async fn fetch_user_records(&self, user: &str) -> anyhow::Result<Arc<UserRecordsPacket>> {
        if let Some(data) = self.user_records_cache.get(user) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request::<UserRecordsPacket>(format!("users/{user}/records")).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_records_cache.insert(user.to_owned(), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.search_user("434626996262273038").await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let user_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(&user_data.user.username);
    /// # });
    /// ```
    /// Invalid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.search_user("abc").await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_none());
    /// # });
    /// ```
    pub async fn search_user(&self, query: &str) -> anyhow::Result<Arc<UserSearchPacket>> {
        if let Some(data) = self.user_search_cache.get(query) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request::<UserSearchPacket>(format!("users/search/{query}")).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_search_cache.insert(query.to_owned(), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    /// # Examples
    /// 
    /// Specify an upper bound:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::After {
    ///     after: OrderedFloat(22000.50), // All users will be below 22000.50, max value is 25000
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(league_data.users.len());
    /// # });
    /// ```
    /// Specify a lower bound: 
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::Before {
    ///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50, max value is 25000
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(league_data.users.len());
    /// # });
    /// ```
    /// Specify a limit without an upper or lower value bound: 
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::NotBound {
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(league_data.users.len());
    /// # });
    /// ```
    /// Specify no settings:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::None).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(league_data.users.len());
    /// # });
    /// ```
    pub async fn fetch_league_leaderboard(&self, query: ValueBoundQuery) -> anyhow::Result<Arc<LeaguePacket>> {
        if let Some(data) = self.league_leaderboard_cache.get(&query) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request(format!("users/lists/league{}", query.as_query_string())).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.league_leaderboard_cache.insert(query, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    /// # Examples
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_full_league_leaderboard(Some("fr".to_string())).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// # });
    /// ```
    pub async fn fetch_full_league_leaderboard(
        &self,
        country: Option<String>,
    ) -> anyhow::Result<Arc<LeagueFullPacket>> {
        if let Some(data) = self.full_league_leaderboard_cache.get(&country) {
            return Ok(Arc::clone(&data));
        }

        let query_string = if let Some(country) = country.clone() {
            format!("?country={country}")
        } else {
            String::new()
        };



        let data = Self::make_tetrio_api_request(format!("users/lists/league/all{}", query_string)).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.full_league_leaderboard_cache.insert(country, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    /// # Examples
    /// 
    /// Specify an upper bound:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::After {
    ///     after: OrderedFloat(22000.50), // All users will be below 22000.50 xp
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let xp_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(xp_data.users.len())
    /// # });
    /// ```
    /// Specify a lower bound: 
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::Before {
    ///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50 xp
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let xp_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(xp_data.users.len());
    /// # });
    /// ```
    /// Specify a limit without an upper or lower value bound: 
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::NotBound {
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()) // A country code
    /// }).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let xp_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(xp_data.users.len());
    /// # });
    /// ```
    /// Specify no settings:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # tokio_test::block_on(async {
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::None).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let xp_data = packet.data.as_ref().unwrap();
    /// 
    /// dbg!(xp_data.users.len());
    /// # });
    /// ```
    pub async fn fetch_xp_leaderboard(&self, query: ValueBoundQuery) -> anyhow::Result<Arc<XpPacket>> {
        if let Some(data) = self.xp_leaderboard_cache.get(&query) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request(format!("users/lists/xp{}", query.as_query_string())).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.xp_leaderboard_cache.insert(query, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    pub async fn fetch_stream(&self, stream: &str) -> anyhow::Result<Arc<StreamPacket>> {
        if let Some(data) = self.stream_cache.get(stream) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request(format!("streams/{stream}")).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.stream_cache.insert(stream.to_owned(), Arc::clone(&data)).await;
        }
        
        Ok(data)
        
    }
    
    pub async fn fetch_news(&self, limit: Option<i64>) -> anyhow::Result<Arc<NewsPacket>> {

        if let Some(data) = self.news_cache.get(&limit) {
            return Ok(Arc::clone(&data));
        }

        let query = if let Some(limit) = limit {
            limit.to_string()
        } else {
            String::new()
        };
        
        let data = Self::make_tetrio_api_request(format!("news/{query}")).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.news_cache.insert(limit, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    pub async fn fetch_latest_news(
        &self,
        stream: &str,
        limit: Option<i64>,
    ) -> anyhow::Result<Arc<LatestNewsPacket>> {

        if let Some(data) = self.latest_news_cache.get(&(stream.to_string(), limit)) {
            return Ok(Arc::clone(&data));
        }

        let query = if let Some(limit) = limit {
            limit.to_string()
        } else {
            String::new()
        };
        
        let data = Self::make_tetrio_api_request(format!("news/{stream}/{query}")).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.latest_news_cache.insert((stream.to_string(), limit), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }

    pub async fn fetch_tetra_league_recent(&self, user_id: &str) -> anyhow::Result<Arc<Packet<LeagueStream>>> {
        if let Some(data) = self.league_stream_cache.get(user_id) {
            return Ok(Arc::clone(&data));
        }

        let data = Self::make_tetrio_api_request(format!("streams/league_userrecent_{user_id}")).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.league_stream_cache.insert(user_id.to_owned(), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
}