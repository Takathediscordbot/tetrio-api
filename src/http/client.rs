use std::fmt::Display;

use crate::models::streams::stream::StreamPacket;
use crate::models::news::NewsPacket;
use crate::http::value_bound_query::ValueBoundQuery;
use crate::models::general::activity::ActivityPacket;
use crate::models::general::stats::StatsPacket;
use crate::models::packet::Packet;
use crate::models::streams::league_stream::LeagueStream;
use crate::models::users::user_info::UserInfoPacket;
use reqwest::header::{HeaderMap, self};
use serde::de::DeserializeOwned;
use crate::models::users::user_records::UserRecordsPacket;
use crate::models::news::latest::LatestNewsPacket;
use crate::models::users::lists::xp::XpPacket;

use crate::models::users::user_search::UserSearchPacket;
use crate::models::users::lists::league::LeaguePacket;
use crate::models::users::lists::league_full::LeagueFullPacket;

pub const TETRIO_API_URL: &str = "https://ch.tetr.io/api/";

async fn make_tetrio_api_request<T: DeserializeOwned>(route: impl Display) -> anyhow::Result<T> {
    let url = format!("{TETRIO_API_URL}{route}");

    Ok(reqwest::get(url).await?.json::<T>().await?)
}

async fn make_tetrio_api_request_with_session_id<T: DeserializeOwned>(route: impl Display, session_id: &str) -> anyhow::Result<T> {
    let url = format!("{TETRIO_API_URL}{route}");
    let mut header_map = HeaderMap::new();
    header_map.insert("X-SESSION-ID",  header::HeaderValue::from_str(session_id)?);  
    let client = reqwest::ClientBuilder::new().default_headers(header_map).build()?;

    Ok(client.get(url).send().await?.json::<T>().await?)
}



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
pub async fn fetch_general_stats() -> anyhow::Result<StatsPacket> {
    make_tetrio_api_request(format!("/general/stats")).await
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
pub async fn fetch_general_activity() -> anyhow::Result<ActivityPacket> {
    make_tetrio_api_request(format!("/general/activity")).await
}

///
/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_user_info("takathedinosaur").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let user_data = packet.data.unwrap();
/// 
/// dbg!(user_data.user.username)
/// # });
/// ```
/// Invalid user:
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_user_info("KAZEOIJAIZDHIQSUDH").await.unwrap();
/// 
/// assert!(!packet.success && packet.data.is_none() && packet.error.is_some());
/// 
/// let error = packet.error.unwrap();
/// 
/// dbg!(error)
/// # });
/// ```
pub async fn fetch_user_info(user: &str) -> anyhow::Result<UserInfoPacket> {
    make_tetrio_api_request(format!("users/{}", user.to_lowercase())).await
}


/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_user_records("takathedinosaur").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let user_data = packet.data.unwrap();
/// 
/// dbg!(user_data.records.blitz)
/// # });
/// ```
/// Invalid user:
/// ```
/// use tetrio_api::http::client;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_user_records("KAZEOIJAIZDHIQSUDH").await.unwrap();
/// 
/// assert!(!packet.success && packet.data.is_none() && packet.error.is_some());
/// 
/// let error = packet.error.unwrap();
/// 
/// dbg!(error)
/// # });
/// ```
pub async fn fetch_user_records(user: &str) -> anyhow::Result<UserRecordsPacket> {
    make_tetrio_api_request(format!("users/{}/records", user.to_lowercase())).await
}

/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::search_user("434626996262273038").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let user_data = packet.data.unwrap();
/// 
/// dbg!(user_data.user.username)
/// # });
/// ```
/// Invalid user:
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::search_user("abc").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_none());
/// # });
/// ```
pub async fn search_user(query: &str) -> anyhow::Result<UserSearchPacket> {
    make_tetrio_api_request(format!("users/search/{query}")).await
}

/// # Examples
/// 
/// Specify an upper bound:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::After {
///     after: OrderedFloat(22000.50), // All users will be below 22000.50, max value is 25000
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()), // A country code
///     session_id: Some("AZERTYUIOP".to_string())
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
/// Specify a lower bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::Before {
///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50, max value is 25000
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()), // A country code
///     session_id: Some("AZERTYUIOP".to_string())
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
/// Specify a limit without an upper or lower value bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::NotBound {
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()) // A country code
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
/// Specify no settings:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::None).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
pub async fn fetch_league_leaderboard(query: ValueBoundQuery) -> anyhow::Result<LeaguePacket> {
    let query_string = query.as_query_string();
    let url = format!("users/lists/league{}", query_string);
    match &query {
        ValueBoundQuery::After { session_id: Some(session_id), .. } => make_tetrio_api_request_with_session_id(url, &session_id).await,
        ValueBoundQuery::Before { session_id: Some(session_id), .. } => make_tetrio_api_request_with_session_id(url, &session_id).await,
        _ => make_tetrio_api_request(url).await
    }
}


/// # Examples
/// ```
/// use tetrio_api::http::client;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_full_league_leaderboard(Some("fr")).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
pub async fn fetch_full_league_leaderboard(
    country: Option<&str>,
) -> anyhow::Result<LeagueFullPacket> {
    let query_string = if let Some(country) = country {
        format!("?country={}", country.to_uppercase())
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("users/lists/league/all{}", query_string)).await
}

/// # Examples
/// 
/// Specify an upper bound:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::After {
///     after: OrderedFloat(22000.50), // All users will be below 22000.50 xp
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()), // A country code
///     session_id: Some("AZERTYUIOP".to_string())
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let xp_data = packet.data.unwrap();
/// 
/// dbg!(xp_data.users.len())
/// # });
/// ```
/// Specify a lower bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::Before {
///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50 xp
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()), // A country code
///     session_id: Some("AZERTYUIOP".to_string())
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let xp_data = packet.data.unwrap();
/// 
/// dbg!(xp_data.users.len())
/// # });
/// ```
/// Specify a limit without an upper or lower value bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::NotBound {
///     limit: Some(50), // Value between 1 and 100
///     country: Some("fr".to_string()) // A country code
/// }).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let xp_data = packet.data.unwrap();
/// 
/// dbg!(xp_data.users.len())
/// # });
/// ```
/// Specify no settings:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::None).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let xp_data = packet.data.unwrap();
/// 
/// dbg!(xp_data.users.len())
/// # });
/// ```
pub async fn fetch_xp_leaderboard(query: ValueBoundQuery) -> anyhow::Result<XpPacket> {
    let query_string = query.as_query_string();
    let url = format!("users/lists/xp{}", query_string);
    match &query {
        ValueBoundQuery::After { session_id: Some(session_id), .. } => make_tetrio_api_request_with_session_id(url, &session_id).await,
        ValueBoundQuery::Before { session_id: Some(session_id), .. } => make_tetrio_api_request_with_session_id(url, &session_id).await,
        _ => make_tetrio_api_request(url).await
    }
}



/// # Examples
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_stream("blitz_userbest_619aaa04dbc55fb324bf4459").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let stream_data = packet.data.unwrap();
/// 
/// // Streams don't have an official documentation
/// // so I can't provide datatypes for that, you'll have to experiment and parse the data yourself
/// dbg!(stream_data);
/// # });
/// ```
pub async fn fetch_stream(stream: &str) -> anyhow::Result<StreamPacket> {
    make_tetrio_api_request(format!("streams/{stream}")).await
}

/// ## WARNING THIS ATTEMPTS TO CONVERT TO A TYPE THAT IS NOT OFFICIALLY SUPPORTED
/// ## IT HAS A HIGH CHANCE OF FAILING DUE TO API CHANGES
/// # Examples
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// # use tetrio_api::delay_test;
/// # tokio_test::block_on(async {
/// # delay_test();
/// let packet = client::fetch_tetra_league_recent("619aaa04dbc55fb324bf4459").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let stream_data = packet.data.unwrap();
/// 
/// // Streams don't have an official documentation
/// // so I can't provide datatypes for that, you'll have to experiment and parse the data yourself
/// dbg!(stream_data);
/// # });
/// ```
pub async fn fetch_tetra_league_recent(user_id: &str) -> anyhow::Result<Packet<LeagueStream>> {
    make_tetrio_api_request(format!("streams/league_userrecent_{user_id}")).await
}


pub async fn fetch_news(limit: Option<i64>) -> anyhow::Result<NewsPacket> {
    let limit = if let Some(limit) = limit {
        format!("?limit={limit}")
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("news{limit}")).await
}

pub async fn fetch_latest_news(
    stream: &str,
    limit: Option<i64>,
) -> anyhow::Result<LatestNewsPacket> {
    let limit = if let Some(limit) = limit {
        format!("?limit={limit}")
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("news/{stream}{limit}")).await
}
