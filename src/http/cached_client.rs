use std::sync::Arc;

use moka::future::Cache;

use super::client::{fetch_full_league_leaderboard, fetch_general_activity, fetch_general_stats, fetch_latest_news, fetch_league_leaderboard, fetch_news, fetch_stream, fetch_tetra_league_recent, fetch_user_info, fetch_user_records, fetch_xp_leaderboard, search_user};
use super::value_bound_query::ValueBoundQuery;
use crate::models::general::activity::ActivityPacket;
use crate::models::general::stats::StatsPacket;
use crate::models::news::latest::LatestNewsPacket;
use crate::models::news::NewsPacket;
use crate::models::packet::{Packet, CacheExpiration};
use crate::models::streams::league_stream::LeagueStream;
use crate::models::streams::stream::StreamPacket;
use crate::models::users::lists::league::LeaguePacket;
use crate::models::users::lists::league_full::LeagueFullPacket;
use crate::models::users::lists::xp::XpPacket;
use crate::models::users::user_info::UserInfoPacket;
use crate::models::users::user_records::UserRecordsPacket;
use crate::models::users::user_search::UserSearchPacket;
use super::error::Error;

/// A TETR.IO Client that automatically caches request results
pub struct CachedClient {
    general_stats_cache: Cache<(), Arc<StatsPacket>>,
    general_activity_cache: Cache<(), Arc<ActivityPacket>>,
    user_info_cache: Cache<Box<str>, Arc<UserInfoPacket>>,
    user_records_cache: Cache<Box<str>, Arc<UserRecordsPacket>>,
    user_search_cache: Cache<Box<str>, Arc<UserSearchPacket>>,
    full_league_leaderboard_cache: Cache<Option<Box<str>>, Arc<LeagueFullPacket>>,
    xp_leaderboard_cache: Cache<ValueBoundQuery, Arc<XpPacket>>,
    league_leaderboard_cache: Cache<ValueBoundQuery, Arc<LeaguePacket>>,
    stream_cache: Cache<Box<str>, Arc<StreamPacket>>,
    league_stream_cache: Cache<Box<str>, Arc<Packet<LeagueStream>>>,
    news_cache: Cache<Option<i64>, Arc<NewsPacket>>,
    latest_news_cache: Cache<(Box<str>, Option<i64>), Arc<LatestNewsPacket>>,
}

impl Default for CachedClient {
    fn default() -> Self {
        Self { 
            general_stats_cache: Cache::builder().expire_after(CacheExpiration).build(),
            general_activity_cache: Cache::builder().expire_after(CacheExpiration).build(),
            user_info_cache: Cache::builder().expire_after(CacheExpiration).build(),
            user_records_cache: Cache::builder().expire_after(CacheExpiration).build(),
            user_search_cache: Cache::builder().expire_after(CacheExpiration).build(),
            league_leaderboard_cache: Cache::builder().expire_after(CacheExpiration).build(),
            xp_leaderboard_cache: Cache::builder().expire_after(CacheExpiration).build(),
            full_league_leaderboard_cache: Cache::builder().expire_after(CacheExpiration).build(),
            stream_cache: Cache::builder().expire_after(CacheExpiration).build(),
            news_cache: Cache::builder().expire_after(CacheExpiration).build(),
            latest_news_cache: Cache::builder().expire_after(CacheExpiration).build(),
            league_stream_cache: Cache::builder().expire_after(CacheExpiration).build()
        }
    }
}


impl CachedClient {

    /// # Examples
    /// 
    /// ```
    /// use tetrio_api::http::client;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let packet = client::fetch_general_stats().await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let general_stats = packet.data.unwrap();
    /// # });
    /// ```
    pub async fn fetch_general_stats(&self) -> Result<Arc<StatsPacket>, Error> {
        if let Some(data) = self.general_stats_cache.get(&()).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_general_stats().await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.general_stats_cache.insert((), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }

    /// # Examples
    /// 
    /// ```
    /// use tetrio_api::http::client;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let packet = client::fetch_general_activity().await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let general_activity = packet.data.unwrap();
    /// # });
    /// ```
    pub async fn fetch_general_activity(&self) -> Result<Arc<ActivityPacket>, Error> {
        if let Some(data) = self.general_activity_cache.get(&()).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_general_activity().await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.general_activity_cache.insert((), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    pub async fn fetch_user_info(&self, user: &str) -> Result<Arc<UserInfoPacket>, Error> {
        let user_boxed = user.to_lowercase().into_boxed_str();
        if let Some(data) = self.user_info_cache.get(&user_boxed).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_user_info(user).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_info_cache.insert(user_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    pub async fn fetch_user_records(&self, user: &str) -> Result<Arc<UserRecordsPacket>, Error> {
        let user_boxed = user.to_lowercase().into_boxed_str();
        if let Some(data) = self.user_records_cache.get(&user_boxed.clone()).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_user_records(user).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_records_cache.insert(user_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    
    /// # Examples
    /// 
    /// Valid user:
    /// ```
    /// use tetrio_api::http::cached_client::CachedClient;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    pub async fn search_user(&self, query: &str) -> Result<Arc<UserSearchPacket>, Error> {
        let query_boxed = query.into();
        if let Some(data) = self.user_search_cache.get(&query_boxed).await {
            return Ok(Arc::clone(&data));
        }

        let data = search_user(query).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.user_search_cache.insert(query_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    /// # Examples
    /// 
    /// Specify an upper bound:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::After {
    ///     after: OrderedFloat(22000.50), // All users will be below 22000.50, max value is 25000
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()), // A country code
    ///     session_id: Some("AZERTYUIOP".to_string()) // A session ID, mainly used for scrolling
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let client = CachedClient::default();
    /// let packet = client.fetch_league_leaderboard(ValueBoundQuery::Before {
    ///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50, max value is 25000
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()), // A country code
    ///     session_id: Some("AZERTYUIOP".to_string()) // A session ID, mainly used for scrolling 
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    pub async fn fetch_league_leaderboard(&self, query: ValueBoundQuery) -> Result<Arc<LeaguePacket>, Error> {
        if let Some(data) = self.league_leaderboard_cache.get(&query).await {
            return Ok(Arc::clone(&data));
        }

        
        
        let data = fetch_league_leaderboard(query.clone()).await;
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let client = CachedClient::default();
    /// let packet = client.fetch_full_league_leaderboard(Some("fr")).await.unwrap();
    /// 
    /// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
    /// 
    /// let league_data = packet.data.as_ref().unwrap();
    /// 
    /// # });
    /// ```
    pub async fn fetch_full_league_leaderboard(
        &self,
        country: Option<&str>,
    ) -> Result<Arc<LeagueFullPacket>, Error> {
        let country_boxed = country.map(|c| c.to_uppercase().into());
        if let Some(data) = self.full_league_leaderboard_cache.get(&country_boxed).await {
            return Ok(Arc::clone(&data));
        }


        let data = fetch_full_league_leaderboard(country).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.full_league_leaderboard_cache.insert(country_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    /// # Examples
    /// 
    /// Specify an upper bound:
    /// ```
    /// use tetrio_api::http::{cached_client::CachedClient, value_bound_query::ValueBoundQuery};
    /// use ordered_float::OrderedFloat;
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::After {
    ///     after: OrderedFloat(22000.50), // All users will be below 22000.50 xp
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()), // A country code
    ///     session_id: Some("AZERTYUIOP".to_string()) // A session ID, mainly used for scrolling
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
    /// let client = CachedClient::default();
    /// let packet = client.fetch_xp_leaderboard(ValueBoundQuery::Before {
    ///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50 xp
    ///     limit: Some(50), // Value between 1 and 100
    ///     country: Some("fr".to_string()), // A country code
    ///     session_id: Some("AZERTYUIOP".to_string()) // A session ID, mainly used for scrolling
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    /// # use tetrio_api::delay_test;
    /// # tokio_test::block_on(async {
    /// # delay_test();
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
    pub async fn fetch_xp_leaderboard(&self, query: ValueBoundQuery) -> Result<Arc<XpPacket>, Error> {
        if let Some(data) = self.xp_leaderboard_cache.get(&query).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_xp_leaderboard(query.clone()).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.xp_leaderboard_cache.insert(query, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
    pub async fn fetch_stream(&self, stream: &str) -> Result<Arc<StreamPacket>, Error> {
        let stream_boxed = stream.into();
        if let Some(data) = self.stream_cache.get(&stream_boxed).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_stream(stream).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.stream_cache.insert(stream_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
        
    }
    
    pub async fn fetch_news(&self, limit: Option<i64>) -> Result<Arc<NewsPacket>, Error> {

        if let Some(data) = self.news_cache.get(&limit).await {
            return Ok(Arc::clone(&data));
        }
        
        let data = fetch_news(limit).await;

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
    ) -> Result<Arc<LatestNewsPacket>, Error> {
        let stream_boxed = stream.to_string().into_boxed_str();
        if let Some(data) = self.latest_news_cache.get(&(stream_boxed.clone(), limit)).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_latest_news(stream, limit).await;

        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.latest_news_cache.insert((stream_boxed, limit), Arc::clone(&data)).await;
        }
        
        Ok(data)
    }

    pub async fn fetch_tetra_league_recent(&self, user_id: &str) -> Result<Arc<Packet<LeagueStream>>, Error> {
        let user_id_boxed = user_id.into();
        if let Some(data) = self.league_stream_cache.get(&user_id_boxed).await {
            return Ok(Arc::clone(&data));
        }

        let data = fetch_tetra_league_recent(user_id).await;
        let result = data.map(Arc::new);
        let Ok(data) = result else {
            return result;
        };

        if data.is_success() {
            self.league_stream_cache.insert(user_id_boxed, Arc::clone(&data)).await;
        }
        
        Ok(data)
    }
    
}