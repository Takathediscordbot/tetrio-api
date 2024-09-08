
 use tetrio_api::{http::{clients::reqwest_client::InMemoryReqwestClient, parameters::{leaderboard_query::LeaderboardType, value_bound_query::{Prisecter, ValueBoundQuery}}}, models::packet::Packet};

 use tokio::runtime::Runtime;

 use std::{fmt::Debug, sync::OnceLock};

 static CLIENT: OnceLock<InMemoryReqwestClient> = OnceLock::new();
 static RUNTIME: OnceLock<Runtime> = OnceLock::new();

 fn get_client() -> &'static InMemoryReqwestClient {
     let client = CLIENT.get_or_init(|| {
         dbg!("CREATED CLIENT AGAIN WEEWOO");
         InMemoryReqwestClient::default()
     });

     client
 }

 fn get_tokio_runtime() -> &'static Runtime {
     let runtime = RUNTIME.get_or_init(|| {
         tokio::runtime::Builder::new_current_thread()
             .enable_all()
             .build()
             .unwrap()
     });

     return runtime;
 }

 fn test_ok_success_is_some<T, E: Debug>(result: Result<Packet<T>, E>) {
     let data = result.expect("Request failed");
     assert_eq!(data.is_success(), true);
     assert_eq!(data.data.is_some(), true);
     assert_eq!(data.cache.is_some(), true);
     assert_eq!(data.error.is_none(), true);
 }

 fn test_ok_failure_is_some<T, E: Debug>(result: Result<Packet<T>, E>) {
     let data = result.expect("Request failed");
     assert_eq!(!data.is_success(), true);
     assert_eq!(data.data.is_none(), true);
     assert_eq!(data.cache.is_some(), false);
     assert_eq!(data.error.is_some(), true);
 }

 fn test_ok_success_is_none<T, E: Debug>(result: Result<Packet<T>, E>) {
     let data = result.expect("Request failed");
     assert_eq!(data.is_success(), true);
     assert_eq!(data.data.is_none(), true);
     assert_eq!(data.cache.is_some(), true);
 }

 #[test]
 fn fetch_general_stats() {
     async fn fetch_general_stats() {
         let client = get_client();
         test_ok_success_is_some(client.fetch_general_stats().await);
         test_ok_success_is_some(client.fetch_general_stats().await);
     }

     get_tokio_runtime().block_on(fetch_general_stats())
 }

 #[test]
 fn fetch_general_activity() {
     async fn fetch_general_activity() {


         let client = get_client();
         test_ok_success_is_some(client.fetch_general_activity().await);
         test_ok_success_is_some(client.fetch_general_activity().await);
     }

     get_tokio_runtime().block_on(fetch_general_activity())
 }

 #[test]
 fn fetch_user_info() {
     async fn fetch_user_info() {


         let client = get_client();
         test_ok_success_is_some(client.fetch_user_info("taka").await);
     }
     get_tokio_runtime().block_on(fetch_user_info())
 }

 #[test]
 fn fetch_founder_info() {
     async fn fetch_founder_info() {


         let client = get_client();
         test_ok_success_is_some(client.fetch_user_info("osk").await);
     }
     get_tokio_runtime().block_on(fetch_founder_info())
 }

 #[test]
 fn fail_fetch_user_info() {
     async fn fail_fetch_user_info() {

         let client = get_client();
         test_ok_failure_is_some(client.fetch_user_info("KJBDEZHDIUZEHDIH").await);
     }
     get_tokio_runtime().block_on(fail_fetch_user_info())
 }

 #[test]
 fn fetch_user_summaries() {
     async fn fetch_user_summaries() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_user_summaries("taka").await);
         test_ok_success_is_some(client.fetch_user_summaries("taka").await);
     }
     get_tokio_runtime().block_on(fetch_user_summaries())
 }

 #[test]
 fn search_discord_user() {
     async fn search_discord_user() {

         let client = get_client();
         test_ok_success_is_some(client.search_discord_user(&"434626996262273038").await);
         test_ok_success_is_some(client.search_discord_user(&"434626996262273038").await);
     }
     get_tokio_runtime().block_on(search_discord_user())
 }

 #[test]
 fn search_invalid_user() {
     async fn search_invalid_user() {

         let client = get_client();
         test_ok_success_is_none(client.search_discord_user(&"IZEGDIHDZ").await);
     }
     get_tokio_runtime().block_on(search_invalid_user())
 }

 #[test]
 fn fail_fetch_user_summaries() {
     async fn fail_fetch_user_summaries() {

         let client = get_client();
         test_ok_failure_is_some(client.fetch_user_summaries("KJBDEZHDIUZEHDIH").await);
     }
     get_tokio_runtime().block_on(fail_fetch_user_summaries())
 }

 #[test]
 fn fetch_league_leaderboard_no_query() {
     async fn fetch_league_leaderboard_no_query() {

         let client = get_client();
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(LeaderboardType::League, ValueBoundQuery::None, None)
                 .await,
         );
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(LeaderboardType::League, ValueBoundQuery::None, None)
                 .await,
         );
     }
     get_tokio_runtime().block_on(fetch_league_leaderboard_no_query())
 }

 #[test]
 fn fetch_leaderboard_country_query() {
     async fn fetch_leaderboard_country_query() {

         let client = get_client();
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::NotBound {
                         limit: None,
                         country: Some("fr".to_string()),
                     },
                     None,
                 )
                 .await,
         );
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::NotBound {
                         limit: None,
                         country: Some("fr".to_string()),
                     },
                     None,
                 )
                 .await,
         );
     }
     get_tokio_runtime().block_on(fetch_leaderboard_country_query())
 }

 #[test]
 fn fetch_leaderboard_after_boundary() {
     async fn fetch_leaderboard_after_boundary() {

         let client = get_client();
         let first_query = client
             .fetch_leaderboard(LeaderboardType::League, ValueBoundQuery::None, None)
             .await
             .unwrap();
         let first_query = first_query.data.unwrap();
         let first_query = first_query.entries.last().unwrap().clone();

         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::After {
                         after: first_query.p,
                         limit: None,
                         country: None,
                     },
                     None,
                 )
                 .await,
         );
     }

     get_tokio_runtime().block_on(fetch_leaderboard_after_boundary())
 }

 #[test]
 fn fetch_leaderboard_before_boundary() {
     async fn fetch_leaderboard_before_boundary() {

         let client = get_client();
         let first_query = client
             .fetch_leaderboard(
                 LeaderboardType::League,
                 ValueBoundQuery::After {
                     after: Prisecter {
                         pri: 200.0,
                         sec: 200.0,
                         ter: 200.0,
                     },
                     limit: None,
                     country: None,
                 },
                 None,
             )
             .await
             .unwrap();

         let first_query = first_query.data.unwrap();
         let first_query = first_query.entries.first().unwrap().clone();

         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::Before {
                         before: first_query.p,
                         limit: None,
                         country: None,
                     },
                     None,
                 )
                 .await,
         );
     }
     get_tokio_runtime().block_on(fetch_leaderboard_before_boundary())
 }

 #[test]
 fn fetch_league_leaderboard_with_session_id() {
     async fn fetch_league_leaderboard_with_session_id() {

         let client = get_client();
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::None,
                     Some("AZERTYUIOP"),
                 )
                 .await,
         );

         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::League,
                     ValueBoundQuery::None,
                     Some("AZERTYUIOP"),
                 )
                 .await,
         );
     }
     get_tokio_runtime().block_on(fetch_league_leaderboard_with_session_id())
 }

 #[test]
 fn fetch_xp_leaderboard_no_query() {
     async fn fetch_xp_leaderboard_no_query() {

         let client = get_client();
         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::Xp,
                     ValueBoundQuery::None,
                     Some("AZERTYUIOP"),
                 )
                 .await,
         );

         test_ok_success_is_some(
             client
                 .fetch_leaderboard(
                     LeaderboardType::Xp,
                     ValueBoundQuery::None,
                     Some("AZERTYUIOP"),
                 )
                 .await,
         );
     }
     get_tokio_runtime().block_on(fetch_xp_leaderboard_no_query())
 } /*

   #[test]
   fn fetch_tetra_league_record()  {
    async fn fetch_tetra_league_record() {
       let client = get_client();
       panic!("hehe");
        Might need to update user id
       test_ok_success_is_some(client.fetch_tetra_league_recent("5e9fb80883e2a23fbb017b04").await);

   }
   get_tokio_runtime().block_on(fetch_tetra_league_record() )
   }

   #[test]
   fn fetch_stream()  {
    async fn fetch_stream() {
       delay_test();
       let client = get_client();
       panic!("hehe");
        Might need to update user id
       test_ok_success_is_some(client.fetch_stream("40l_userbest_5e9fb80883e2a23fbb017b04").await);
       delay_test();
       test_ok_success_is_some(client.fetch_stream("blitz_userbest_5e9fb80883e2a23fbb017b04").await);
       delay_test();
       test_ok_success_is_some(client.fetch_stream("any_userrecent_5e9fb80883e2a23fbb017b04").await);
       delay_test();
       test_ok_success_is_some(client.fetch_stream("40l_global").await);
       delay_test();
       test_ok_success_is_some(client.fetch_stream("blitz_global").await);
       test_ok_success_is_some(client.fetch_stream("40l_userbest_5e9fb80883e2a23fbb017b04").await);
       test_ok_success_is_some(client.fetch_stream("blitz_userbest_5e9fb80883e2a23fbb017b04").await);
       test_ok_success_is_some(client.fetch_stream("any_userrecent_5e9fb80883e2a23fbb017b04").await);
       test_ok_success_is_some(client.fetch_stream("40l_global").await);
       test_ok_success_is_some(client.fetch_stream("blitz_global").await);
   }
   get_tokio_runtime().block_on(fetch_stream() )
   }*/

 #[test]
 fn fetch_news() {
     async fn fetch_news() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_news(None).await);
         test_ok_success_is_some(client.fetch_news(None).await);
     }
     get_tokio_runtime().block_on(fetch_news())
 }

 #[test]
 fn fetch_latest_news() {
     async fn fetch_latest_news() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_latest_news("global", None).await);
         test_ok_success_is_some(client.fetch_latest_news("global", None).await);
     }
     get_tokio_runtime().block_on(fetch_latest_news())
 }

 #[test]
 fn fetch_news_with_limit() {
     async fn fetch_news_with_limit() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_news(Some(10)).await);
         test_ok_success_is_some(client.fetch_news(Some(10)).await);
     }
     get_tokio_runtime().block_on(fetch_news_with_limit())
 }

 #[test]
 fn fetch_latest_news_with_limit() {
     async fn fetch_latest_news_with_limit() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_latest_news("global", Some(10)).await);
         test_ok_success_is_some(client.fetch_latest_news("global", Some(10)).await);
     }
     get_tokio_runtime().block_on(fetch_latest_news_with_limit())
 }


 #[test]
 fn fetch_scoreflow() {
     async fn fetch_scoreflow() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_scoreflow("taka", "40l").await);
         test_ok_success_is_some(client.fetch_scoreflow("taka", "40l").await);
     }
     get_tokio_runtime().block_on(fetch_scoreflow())
 }

 #[test]
 fn fetch_leagueflow() {
     async fn fetch_leagueflow() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_leagueflow("taka").await);
         test_ok_success_is_some(client.fetch_leagueflow("taka").await);
     }
     get_tokio_runtime().block_on(fetch_leagueflow())
 }

 
 #[test]
 fn fetch_leagueranks() {
     async fn fetch_leagueranks() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_leagueranks().await);
         test_ok_success_is_some(client.fetch_leagueranks().await);
     }
     get_tokio_runtime().block_on(fetch_leagueranks())
 }

 #[test]
 fn fetch_achievement_info() {
     async fn fetch_achievement_info() {

         let client = get_client();
         test_ok_success_is_some(client.fetch_achievement_info("2").await);
         test_ok_success_is_some(client.fetch_achievement_info("2").await);
     }
     get_tokio_runtime().block_on(fetch_achievement_info())
 }