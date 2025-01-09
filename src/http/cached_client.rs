
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use async_lock::Mutex;
use bytes::{Buf, Bytes};
use futures::FutureExt;
use futures_core::future::BoxFuture;
use http::{HeaderValue, Request};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;
use super::caches::cache::CacheHandler;
use super::error::{Error, ErrorTrait};
use super::clients::http_client::HttpClient;
use super::parameters::leaderboard_query::LeaderboardType;
use super::parameters::personal_user_records::{PersonalLeaderboard, PersonalRecordsQuery};
use super::parameters::value_bound_query::ValueBoundQuery;
use crate::models::general::achivement_info::AchievementInfoPacket;
use crate::models::general::activity::ActivityPacket;
use crate::models::general::stats::StatsPacket;
use crate::models::labs::league_ranks::LeagueRanksPacket;
use crate::models::labs::leagueflow::LeagueFlowPacket;
use crate::models::labs::scoreflow::ScoreFlowPacket;
use crate::models::news::latest::LatestNewsPacket;
use crate::models::news::NewsPacket;
use crate::models::packet::{Packet, SuccessPacket};
use crate::models::users::summaries::{AchievementsSummaryPacket, AllSummariesPacket, BlitzSummaryPacket, LeagueSummaryPacket, SprintSummaryPacket, ZenSummaryPacket, ZenithExSummaryPacket, ZenithSummaryPacket};
use crate::models::users::user_history_leaderboard::HistoricalLeaderboardPacket;
use crate::models::users::user_info::UserInfoPacket;
use crate::models::users::user_leaderboard::LeaderboardPacket;
use crate::models::users::user_records::{PersonalBlitzRecordPacket, PersonalLeagueRecordPacket, PersonalSprintRecordPacket, PersonalZenithExRecordPacket, PersonalZenithRecordPacket};
use crate::models::users::user_search::UserSearchPacket;
use futures::future::Either;
use tower::Service;
use tower_util::ServiceExt;

pub struct CachedClient<HttpClientImpl: HttpClient, Cache: CacheHandler<HttpClientImpl::HttpError>> {
    req_service: Mutex<Box<dyn Send + Sync + Service< Request<Vec<u8>>, Response = Bytes, Error = HttpClientImpl::HttpError, Future = BoxFuture<'static, Result<Bytes, HttpClientImpl::HttpError>>>>>,
    cache_handler: Cache,
    _phantom: PhantomData<HttpClientImpl>,
}

impl<HttpClientImpl: HttpClient + Default, Cache: CacheHandler<HttpClientImpl::HttpError> + Default> Default for CachedClient<HttpClientImpl, Cache> {
    fn default() -> Self {
        return Self::new(HttpClientImpl::default(), Cache::default())
    }
}


impl<HttpClientImpl: HttpClient, Cache: CacheHandler<HttpClientImpl::HttpError>> CachedClient<HttpClientImpl, Cache> {
    pub fn new(client: HttpClientImpl, cache_handler: Cache) -> Self {
            let client = Arc::new(client);
            let svc = tower::ServiceBuilder::new()
                .rate_limit(1, Duration::new(1, 0)) // 1 requests every 1 seconds
                .service(tower::service_fn(move |request| {
                    let clone_client = client.clone();
                    async move {
                        clone_client.execute(request).await
                    }.boxed()
                }));

            CachedClient {
                req_service: Mutex::new(Box::new(svc)),
                cache_handler,
                _phantom: PhantomData::default(),
            }
    }

}




pub const TETRIO_API_URL: &str = "https://ch.tetr.io/api/";

impl<HttpClientImpl: HttpClient + Send + Sync, Cache: CacheHandler<HttpClientImpl::HttpError>> ErrorTrait for CachedClient<HttpClientImpl, Cache> {
    type Error = Error<HttpClientImpl::HttpError, Cache::CachingError>;
}

impl<HttpClientImpl: HttpClient + Send + Sync, Cache: CacheHandler<HttpClientImpl::HttpError>> CachedClient<HttpClientImpl, Cache> {

   
    pub(crate) async fn parse_body<T: DeserializeOwned>(body: Bytes) -> Result<T, <Self as ErrorTrait>::Error> {
        let jd = &mut serde_json::Deserializer::from_reader(body.clone().reader());

        let result: Result<T, _> = serde_path_to_error::deserialize(jd);

        match result {
            Ok(v) => Ok(v),
            Err(err) => {
                Err(Error::ParsingError(err))
            }
        }
    }

    pub fn make_url<T: Display>(href: &str, query_params: &[[T; 2]]) -> String {
        let mut url = match Url::from_str(&format!("{TETRIO_API_URL}{href}")) {
            Ok(v) => v,
            Err(_) => unreachable!() // TETRIO_API_URL HAS TO BE VALID for ANYTHING to work.
        };
    
    
        {
            let mut query = url.query_pairs_mut();
    
            for pair in query_params {
                query.append_pair(&pair[0].to_string(), &pair[1].to_string());
            };
    
            query.finish();
        }
    
    
    
        return dbg!(url.to_string().replacen(TETRIO_API_URL, "", 1));
    }

    pub async fn make_request<T: DeserializeOwned + Serialize>(&self, url: &str, session_id: &Option<&str>) -> Result<T, <Self as ErrorTrait>::Error>  {
        let req = Request::builder()
                        .method(http::Method::GET)
                        .uri(url);

        let req = if let Some(session_id) = session_id {
            req.header("X-SESSION-ID", HeaderValue::from_str(session_id).map_err(Error::InvalidHeaderValue)?)
        } else {
            req
        };
        

        let mut service = self.req_service.lock().await;

        let response = service.ready_and().await.map_err(Error::HttpError)?.call(req.body(vec![]).map_err(Error::RequestParsingError)?).await.map_err(Error::HttpError)?;

        

        Self::parse_body::<T>(
            response
        ).await
    }

    pub async fn cache_value_if_success<T: DeserializeOwned + Serialize + Send + Sync>(&self, cache_key: String, r: Packet<T>) -> Result<bool, <Self as ErrorTrait>::Error> {
        let Packet { success, cache, data, .. } = r;
        match (cache, success, data) {
            (Some(cache), true, Some(data)) => {
                self.cache_handler.cache_value(&cache_key, SuccessPacket { success: true, cache, data }).await?;

                Ok(true)
            }
            _ => { Ok(false) }
        }
    }

    pub(crate) fn get_url(route: impl Display) -> String {
        format!("{TETRIO_API_URL}{route}")
    }

    pub(crate) fn get_cache_key(url: &str, session_id: &Option<&str>) -> String {
        format!("{url}&X_SESSION_ID={session_id:?}")
    }

    pub async fn make_tetrio_api_request<T: DeserializeOwned + Serialize + Send + Sync + Clone>(&self, route: impl Display, session_id: Option<&str>) -> Result<Packet<T>, <Self as ErrorTrait>::Error> {
        let url = Self::get_url(route);
        let cache_key = Self::get_cache_key(&url, &session_id);
        dbg!(&url);
        let response = self.cache_handler.try_get_cache(&cache_key).await?;
        response.map_or_else(|| Either::Left(async {
            let packet = self.make_request::<Packet<T>>(&url, &session_id).await;
            dbg!(packet.is_ok());
            // ignore error because we don't care if it's not cached
            let _ = match &packet {
                Ok(value) => {self.cache_value_if_success(cache_key, value.clone()).await},
                _ => { return packet; }
            };

            return packet;
        }), |value| Either::Right(async move { Ok(value) })).await
    }

    pub async fn cache_tetrio_api_result_if_not_present<T: DeserializeOwned + Serialize + Clone + Send + Sync>(&self, route: impl Display, session_id: Option<&str>, packet: &str) -> Result<Packet<T>, <Self as ErrorTrait>::Error> {
        let url = Self::get_url(route);
        let cache_key = Self::get_cache_key(&url, &session_id);
        let response = self.cache_handler.try_get_cache(&cache_key).await?;
        response.map_or_else(|| Either::Left(async {
            let deserializer = &mut serde_json::Deserializer::from_str(packet);

            let result: Result<Packet<T>, _> = serde_path_to_error::deserialize(deserializer).map_err(
                Error::ConversionError
            );


            // ignore error because we don't care if it's not cached
            let _ = match &result {
                Ok(value) => {self.cache_value_if_success(cache_key, value.clone()).await},
                _ => { return result }
            };

            return result;
        }), |value| Either::Right(async move { Ok(value) })).await
    }

    pub async fn get_from_cache<T: DeserializeOwned + Serialize + Clone + Send + Sync>(&self, route: impl Display, session_id: Option<&str>) -> Result<Option<Packet<T>>, <Self as ErrorTrait>::Error> {
        let url = Self::get_url(route);
        let cache_key = Self::get_cache_key(&url, &session_id);
    
        self.cache_handler.try_get_cache(&cache_key).await
    }



















    pub async fn fetch_general_stats(&self) -> Result<StatsPacket, <Self as ErrorTrait>::Error> {
        return self.make_tetrio_api_request("general/stats", None).await;
    }















    pub async fn fetch_general_activity(&self) -> Result<ActivityPacket, <Self as ErrorTrait>::Error> {
        return self.make_tetrio_api_request("general/activity", None).await;

    }
    
































    pub async fn fetch_user_info(&self, user: &str) -> Result<UserInfoPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}"), None).await
    }
    


































    pub async fn fetch_user_summaries(&self, user: &str) -> Result<AllSummariesPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries"), None).await

    }



































    pub async fn fetch_user_zen_summaries(&self, user: &str) -> Result<ZenSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/zen"), None).await

    }



































    pub async fn fetch_user_league_summaries(&self, user: &str) -> Result<LeagueSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/league"), None).await

    }



































    pub async fn fetch_user_40l_summaries(&self, user: &str) -> Result<SprintSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/40l"), None).await

    }




































    pub async fn fetch_user_blitz_summaries(&self, user: &str) -> Result<BlitzSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/blitz"), None).await

    }



































    pub async fn fetch_user_zenith_summaries(&self, user: &str) -> Result<ZenithSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/zenith"), None).await

    }



































    pub async fn fetch_user_zenithex_summaries(&self, user: &str) -> Result<ZenithExSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/zenithex"), None).await

    }



































    pub async fn fetch_user_achievements_summaries(&self, user: &str) -> Result<AchievementsSummaryPacket, <Self as ErrorTrait>::Error> {
        self.make_tetrio_api_request(format!("users/{user}/summaries/achievements"), None).await
    }
    
    




























    pub async fn search_discord_user(&self, query: &str) -> Result<UserSearchPacket, <Self as ErrorTrait>::Error> {


        self.make_tetrio_api_request(format!("users/search/discord:{query}"), None).await
    }

    pub async fn fetch_leaderboard(&self,
                                   leaderboard_type: LeaderboardType,
                                   query: ValueBoundQuery,
                                   session_id: Option<&str>) -> Result<LeaderboardPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/by/{}", leaderboard_type.to_string());
        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), session_id).await
    }

    pub async fn fetch_historical_leaderboard(&self,
                                   leaderboard_type: LeaderboardType,
                                   season: String,
                                   query: ValueBoundQuery,
                                   session_id: Option<&str>) -> Result<HistoricalLeaderboardPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/history/{}/{}", leaderboard_type.to_string(), season);
        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), session_id).await
    }

    pub async fn fetch_user_personal_40l_records(&self,
                                             user: &str,
                                             leaderboard: PersonalLeaderboard,
                                             query: PersonalRecordsQuery) -> Result<PersonalSprintRecordPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/{}/records/{}/{}", user, "40l", leaderboard.to_string());

        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), None).await
    }

    pub async fn fetch_user_personal_blitz_records(&self,
                                             user: &str,
                                             leaderboard: PersonalLeaderboard,
                                             query: PersonalRecordsQuery) -> Result<PersonalBlitzRecordPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/{}/records/{}/{}", user, "blitz", leaderboard.to_string());

        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), None).await
    }

    pub async fn fetch_user_personal_league_records(&self,
                                             user: &str,
                                             leaderboard: PersonalLeaderboard,
                                             query: PersonalRecordsQuery) -> Result<PersonalLeagueRecordPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/{}/records/{}/{}", user, "league", leaderboard.to_string());

        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), None).await
    }

    pub async fn fetch_user_personal_zenith_records(&self,
                                             user: &str,
                                             leaderboard: PersonalLeaderboard,
                                             query: PersonalRecordsQuery) -> Result<PersonalZenithRecordPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/{}/records/{}/{}", user, "zenith", leaderboard.to_string());

        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), None).await
    }

    pub async fn fetch_user_personal_zenithex_records(&self,
                                             user: &str,
                                             leaderboard: PersonalLeaderboard,
                                             query: PersonalRecordsQuery) -> Result<PersonalZenithExRecordPacket, <Self as ErrorTrait>::Error> {
        let url = format!("users/{}/records/{}/{}", user, "zenithex", leaderboard.to_string());

        self.make_tetrio_api_request(Self::make_url(&url, &query.as_query_params()), None).await
    }
    
    pub async fn fetch_news(&self, limit: Option<i64>) -> Result<NewsPacket, <Self as ErrorTrait>::Error> {
        let url = "news/";
        let limit = limit.map(|l| vec![["limit".to_lowercase(), l.to_string()]]).unwrap_or(vec![]);
        self.make_tetrio_api_request(Self::make_url(&url, &limit), None).await

    }
    
    pub async fn fetch_latest_news(
        &self,
        stream: &str,
        limit: Option<i64>,
    ) -> Result<LatestNewsPacket, <Self as ErrorTrait>::Error> {
        let url = format!("news/{}", stream);
        let limit = limit.map(|l| vec![["limit".to_lowercase(), l.to_string()]]).unwrap_or(vec![]);
        self.make_tetrio_api_request(Self::make_url(&url, &limit), None).await
    }
    
    pub async fn fetch_scoreflow(&self, user: &str, game_mode: &str) -> Result<ScoreFlowPacket, <Self as ErrorTrait>::Error> {
        let url = format!("labs/scoreflow/{user}/{game_mode}");
        self.make_tetrio_api_request(url, None).await
    }

    pub async fn fetch_leagueflow(&self, user: &str) -> Result<LeagueFlowPacket, <Self as ErrorTrait>::Error> {
        let url = format!("labs/leagueflow/{user}");
        self.make_tetrio_api_request(url, None).await
    }

    pub async fn fetch_leagueranks(&self) -> Result<LeagueRanksPacket, <Self as ErrorTrait>::Error> {
        let url = format!("labs/league_ranks");
        self.make_tetrio_api_request(url, None).await
    }

    pub async fn fetch_achievement_info(&self, achievement: &str) -> Result<AchievementInfoPacket, <Self as ErrorTrait>::Error> {
        let url = format!("achievements/{achievement}");
        self.make_tetrio_api_request(url, None).await
    }
}

