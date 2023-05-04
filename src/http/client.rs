use crate::http::value_bound_query::ValueBoundQuery;
use crate::models::packet::Packet;
use crate::models::streams::league_stream::LeagueStream;
use serde::de::DeserializeOwned;

pub const TETRIO_API_URL: &str = "https://ch.tetr.io/api/";

async fn make_tetrio_api_request<T: DeserializeOwned>(route: String) -> anyhow::Result<T> {
    let url = format!("{TETRIO_API_URL}{route}");

    Ok(reqwest::get(url).await?.json::<T>().await?)
}


use crate::models::users::user_info::UserInfoPacket;

///
/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # tokio_test::block_on(async {
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
/// # tokio_test::block_on(async {
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
    make_tetrio_api_request(format!("users/{user}")).await
}

use crate::models::users::user_records::UserRecordsPacket;

/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # tokio_test::block_on(async {
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
    make_tetrio_api_request(format!("users/{user}/records")).await
}

use crate::models::users::user_search::UserSearchPacket;
/// # Examples
/// 
/// Valid user:
/// ```
/// use tetrio_api::http::client;
/// # tokio_test::block_on(async {
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
/// # tokio_test::block_on(async {
/// let packet = client::search_user("abc").await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_none());
/// # });
/// ```
pub async fn search_user(query: &str) -> anyhow::Result<UserSearchPacket> {
    make_tetrio_api_request(format!("users/search/{query}")).await
}

use crate::models::users::lists::league::LeaguePacket;
/// # Examples
/// 
/// Specify an upper bound:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::After {
///     after: OrderedFloat(22000.50), // All users will be below 22000.50, max value is 25000
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
/// Specify a lower bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_league_leaderboard(ValueBoundQuery::Before {
///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50, max value is 25000
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
/// Specify a limit without an upper or lower value bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
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
    make_tetrio_api_request(format!("users/lists/league{}", query.as_query_string())).await
}


use crate::models::users::lists::league_full::LeagueFullPacket;
/// # Examples
/// ```
/// use tetrio_api::http::client;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_full_league_leaderboard(Some("fr".to_string())).await.unwrap();
/// 
/// assert!(packet.success && packet.data.is_some() && packet.error.is_none());
/// 
/// let league_data = packet.data.unwrap();
/// 
/// dbg!(league_data.users.len())
/// # });
/// ```
pub async fn fetch_full_league_leaderboard(
    country: Option<String>,
) -> anyhow::Result<LeagueFullPacket> {
    let query_string = if let Some(country) = country {
        format!("?country={country}")
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("users/lists/league/all{}", query_string)).await
}

use crate::models::users::lists::xp::XpPacket;
/// # Examples
/// 
/// Specify an upper bound:
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::After {
///     after: OrderedFloat(22000.50), // All users will be below 22000.50 xp
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
/// Specify a lower bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
/// let packet = client::fetch_xp_leaderboard(ValueBoundQuery::Before {
///     before: OrderedFloat(22000.50), // All users will be higher than 22000.50 xp
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
/// Specify a limit without an upper or lower value bound: 
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// use ordered_float::OrderedFloat;
/// # tokio_test::block_on(async {
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
/// # tokio_test::block_on(async {
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
    make_tetrio_api_request(format!("users/lists/xp{}", query.as_query_string())).await
}



use crate::models::streams::stream::StreamPacket;
/// # Examples
/// ```
/// use tetrio_api::http::{client, value_bound_query::ValueBoundQuery};
/// # tokio_test::block_on(async {
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
/// # tokio_test::block_on(async {
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


use crate::models::news::news::NewsPacket;
pub async fn fetch_news(limit: Option<i64>) -> anyhow::Result<NewsPacket> {
    let limit = if let Some(limit) = limit {
        limit.to_string()
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("news/{limit}")).await
}

use crate::models::news::latest::LatestNewsPacket;
pub async fn fetch_latest_news(
    stream: &str,
    limit: Option<i64>,
) -> anyhow::Result<LatestNewsPacket> {
    let limit = if let Some(limit) = limit {
        limit.to_string()
    } else {
        String::new()
    };
    make_tetrio_api_request(format!("news/{stream}/{limit}")).await
}
