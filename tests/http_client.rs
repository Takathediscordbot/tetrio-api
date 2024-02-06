use std::fmt::Debug;
use std::time::Duration;

/// This is only used to make tests not get rate limited. It needs to be public to be accessed by doctest
pub fn delay_test() {
    std::thread::sleep(Duration::from_millis(500));
}

use tetrio_api::http::value_bound_query::ValueBoundQuery;

use tetrio_api::http::client;
use tetrio_api::models::packet::Packet;    

fn test_ok_success_is_some<T, E: Debug>(result: Result<Packet<T>, E>) {
    let data = result.  expect("Request failed");
    assert_eq!(data.is_success(), true);
    assert_eq!(data.data.is_some(), true);
    assert_eq!(data.cache.is_some(), true);
    assert_eq!(data.error.is_none(), true);
}

fn test_ok_failure_is_some<T, E: Debug>(result: Result<Packet<T>, E>) {
    let data = result.expect("Request failed");
    assert_eq!(!data.is_success(), true);
    assert_eq!(data.data.is_none(), true);
    assert_eq!(data.cache.is_some(), true);
    assert_eq!(data.error.is_some(), true);
}


fn test_ok_success_is_none<T, E: Debug>(result: Result<Packet<T>, E>) {
    let data = result.expect("Request failed");
    assert_eq!(data.is_success(), true);
    assert_eq!(data.data.is_none(), true);
    assert_eq!(data.cache.is_some(), true);
}

#[tokio::test] 
async fn fetch_general_stats() {
    delay_test();

    test_ok_success_is_some(client::fetch_general_stats().await);
}

#[tokio::test] 
async fn fetch_general_activity() {
    delay_test();

    test_ok_success_is_some(client::fetch_general_activity().await);
}


#[tokio::test]
async fn fetch_user_info() {
    delay_test();
    test_ok_success_is_some(client::fetch_user_info("takathedinosaur").await);
}

#[tokio::test]
async fn fail_fetch_user_info() {
    delay_test();
    test_ok_failure_is_some(client::fetch_user_info("KJBDEZHDIUZEHDIH").await);

}

#[tokio::test]
async fn fetch_user_records() {
    // delay_test();
    // test_ok_success_is_some(client::fetch_user_records("takathedinosaur").await);
    // delay_test();
    // test_ok_success_is_some(client::fetch_user_records("icly").await);
    // delay_test();
    // test_ok_success_is_some(client::fetch_user_records("toasty").await);
    delay_test();
    test_ok_success_is_some(client::fetch_user_records("osk").await);
    // delay_test();
    // test_ok_success_is_some(client::fetch_user_records("kagari").await);
}

#[tokio::test]
async fn fail_fetch_user_records() {
    delay_test();
    test_ok_failure_is_some(client::fetch_user_records("KJBDEZHDIUZEHDIH").await);
}

#[tokio::test]
async fn search_user() {
    delay_test();
    test_ok_success_is_some(client::search_user(&"434626996262273038").await);
}

#[tokio::test]
async fn search_invalid_user() {
    delay_test();
    test_ok_success_is_none(client::search_user(&"IZEGDIHDZ").await);
}

#[tokio::test]
async fn fetch_leaderboard_no_query() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(ValueBoundQuery::None).await);
}

#[tokio::test]
async fn fetch_leaderboard_country_query() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(
        ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
    ).await);

}

#[tokio::test]
async fn fetch_leaderboard_after_boundary() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(ValueBoundQuery::After { 
        after: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: None
    }).await);

}

#[tokio::test]
async fn fetch_leaderboard_before_boundary() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(
        ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: None }
    ).await);
}

#[tokio::test]
async fn fetch_league_leaderboard_after_boundary_with_session_id() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(ValueBoundQuery::After { 
        after: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: Some("AZERTYUIOP".to_string())
    }).await);
}

#[tokio::test]
async fn fetch_league_leaderboard_before_boundary_with_session_id() {
    delay_test();
    test_ok_success_is_some(client::fetch_league_leaderboard(
        ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: Some("AZERTYUIOP".to_string()) }
    ).await);

}

#[tokio::test]
async fn fetch_xp_leaderboard_no_query() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(ValueBoundQuery::None).await);
}

#[tokio::test]
async fn fetch_xp_leaderboard_country_query() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(
        ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
    ).await);

}

#[tokio::test]
async fn fetch_xp_leaderboard_after_boundary() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(ValueBoundQuery::After { 
        after: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: None
    }).await);
}

#[tokio::test]
async fn fetch_xp_leaderboard_before_boundary() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(
        ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: None }
    ).await);

}

#[tokio::test]
async fn fetch_xp_leaderboard_after_boundary_with_session_id() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(ValueBoundQuery::After { 
        after: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: Some("AZERTYUIOP".to_string())
    }).await);
}

#[tokio::test]
async fn fetch_xp_leaderboard_before_boundary_with_session_id() {
    delay_test();
    test_ok_success_is_some(client::fetch_xp_leaderboard(
        ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None, session_id: Some("AZERTYUIOP".to_string()) }
    ).await);

}

#[tokio::test]
#[ignore = "Test takes too long"]
async fn fetch_full_league_leaderboard() {
    delay_test();
    test_ok_success_is_some(client::fetch_full_league_leaderboard(None).await);
}   

#[tokio::test]
async fn fetch_full_league_leaderboard_country() {
    delay_test();
    test_ok_success_is_some(client::fetch_full_league_leaderboard(Some("fr")).await);
}   

#[tokio::test]
async fn fetch_tetra_league_record() {

    // Might need to update user id
    test_ok_success_is_some(client::fetch_tetra_league_recent("5e9fb80883e2a23fbb017b04").await); 

}

#[tokio::test]
async fn fetch_stream() {
    delay_test();

    // Might need to update user id
    test_ok_success_is_some(client::fetch_stream("40l_userbest_5e9fb80883e2a23fbb017b04").await); 
    delay_test();
    test_ok_success_is_some(client::fetch_stream("blitz_userbest_5e9fb80883e2a23fbb017b04").await);
    delay_test();
    test_ok_success_is_some(client::fetch_stream("any_userrecent_5e9fb80883e2a23fbb017b04").await); 
    delay_test();
    test_ok_success_is_some(client::fetch_stream("40l_global").await);
    delay_test();
}

#[tokio::test]
async fn fetch_news() {
    delay_test();
    test_ok_success_is_some(client::fetch_news(None).await);
}

#[tokio::test]
async fn fetch_latest_news() {
    delay_test();
    test_ok_success_is_some(client::fetch_latest_news("global", None).await);
}

#[tokio::test]
async fn fetch_news_with_limit() {
    delay_test();

    test_ok_success_is_some(client::fetch_news(Some(10)).await);
    test_ok_success_is_some(client::fetch_news(Some(10)).await);
}

#[tokio::test]
async fn fetch_latest_news_with_limit() {
    delay_test();

    test_ok_success_is_some(client::fetch_latest_news("global", Some(10)).await);
    test_ok_success_is_some(client::fetch_latest_news("global", Some(10)).await);
}