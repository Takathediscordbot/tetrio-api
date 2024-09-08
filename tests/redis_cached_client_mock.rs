#![cfg(feature = "reqwest_http_client")]
#![cfg(feature = "redis_cache")]


use std::fs;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tetrio_api::http::caches::redis_cache::RedisCache;
use tetrio_api::http::clients::reqwest_client::RedisReqwestClient;

use tetrio_api::models::packet::Packet;
use tetrio_api::models::users::summaries::{
    AllSummariesPacket, BlitzSummaryPacket, LeagueSummaryPacket, SprintSummaryPacket,
    ZenSummaryPacket, ZenithExSummaryPacket, ZenithSummaryPacket,
};
use tetrio_api::models::users::user_info::UserInfoPacket;
use tetrio_api::models::users::user_records::{
    PersonalBlitzRecordPacket, PersonalLeagueRecordPacket, PersonalSprintRecordPacket,
    PersonalZenithExRecordPacket, PersonalZenithRecordPacket,
};
use tokio::runtime::Runtime;

use std::sync::OnceLock;

static CLIENT: OnceLock<RedisReqwestClient<'static>> = OnceLock::new();
static RUNTIME: OnceLock<Runtime> = OnceLock::new();
static REDIS_CONNECTION: OnceLock<redis::Client> = OnceLock::new();

fn get_redis_connection() -> &'static redis::Client {
    REDIS_CONNECTION.get_or_init(|| {
        redis::Client::open("redis://127.0.0.1/").unwrap()
    })
}

fn get_client() -> &'static RedisReqwestClient<'static> {
    let client = CLIENT.get_or_init(|| {
        dbg!("CREATED CLIENT AGAIN WEEWOO");
        RedisReqwestClient::new(Default::default(), RedisCache::new(std::borrow::Cow::Borrowed(get_redis_connection())))
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

fn to_packet<T: DeserializeOwned + Clone + Serialize>(data: T) -> Packet<T> {
    Packet {
        success: true,
        cache: None,
        data: Some(data),
        error: None,
    }
}

fn read_to_packet(path: &str) -> serde_json::Value {
    let data = fs::read_to_string(path).unwrap();
    serde_json::to_value(to_packet::<Vec<serde_json::Value>>(
        serde_json::from_str(&data).unwrap(),
    ))
    .unwrap()
}

#[test]
fn fetch_user_info() {
    async fn fetch_user_info() {
        let path = "./tests/data/tetrio_users_infos.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<UserInfoPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user infos!");
    }
    get_tokio_runtime().block_on(fetch_user_info())
}

#[test]
fn fetch_user_summary() {
    async fn fetch_user_summary() {
        let path = "./tests/data/tetrio_users_all_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<AllSummariesPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_40l_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<SprintSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_blitz_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<BlitzSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_league_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<LeagueSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_zen_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<ZenSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_zenith_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<ZenithSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");

        let path = "./tests/data/tetrio_users_zenithex_summaries.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<ZenithExSummaryPacket>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");
    }
    get_tokio_runtime().block_on(fetch_user_summary())
}

#[test]
fn fetch_user_records() {
    async fn fetch_user_records_<T: DeserializeOwned + Clone + Serialize + Send + Sync>(record_type: &str) {
        let record_modifiers = ["progression", "recent", "top"];
        for record_modifier in record_modifiers {
            let file =
                format!("./tests/data/tetrio_users_{record_type}_records_{record_modifier}.json");
            let client = get_client();
            eprintln!("Parsing {record_type}");
            client
                .cache_tetrio_api_result_if_not_present::<Vec<T>>(&file, None, read_to_packet(&file))
                .await
                .expect("Couldn't parse user summaries!");
        }
    }

    async fn fetch_user_records_test() {
        fetch_user_records_::<PersonalSprintRecordPacket>("40l").await;
        fetch_user_records_::<PersonalBlitzRecordPacket>("blitz").await;
        fetch_user_records_::<PersonalLeagueRecordPacket>("league").await;
        fetch_user_records_::<PersonalZenithRecordPacket>("zenith").await;
        fetch_user_records_::<PersonalZenithExRecordPacket>("zenithex").await;
    }

    get_tokio_runtime().block_on(fetch_user_records_test())
}

#[test]
fn fetch_league_leaderboard() {
    async fn fetch_league_leaderboard() {

        let path = "./tests/data/tetrio_full_league_leaderboard.json";
        let client = get_client();

        client
            .cache_tetrio_api_result_if_not_present::<Vec<Vec<tetrio_api::models::users::user_leaderboard::LeaderboardUser>>>(path, None, read_to_packet(path))
            .await
            .expect("Couldn't parse user summaries!");
    }

    get_tokio_runtime().block_on(fetch_league_leaderboard())

}
