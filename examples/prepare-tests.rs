use std::{collections::HashMap, fs, future::Future};

use futures::future::join_all;
use redis::Value;
use serde::Serialize;
use serde_json::json;
use tetrio_api::{http::{cached_client::CachedClient, caches::moka::MokaCache, clients::reqwest_client::{InMemoryReqwestClient, InMemoryReqwestError, ReqwestClient}, parameters::value_bound_query::{Prisecter, ValueBoundQuery}}, models::packet::Packet};

const TETRIO_USERS: [[&'static str; 2]; 50] = [
    [
      "619aaa04dbc55fb324bf4459",
      "taka"
    ],
    [
      "5e7cbb652932b46c9c671ce4",
      "icly"
    ],
    [
      "5e32fc85ab319c2ab1beb07c",
      "osk"
    ],
    [
      "615fb20fe17beeef45104302",
      "blaarg"
    ],
    [
      "5e331c3ce24a5a3e258f7a1b",
      "kagari"
    ],
    [
      "5e98b21e77ea8f7e30009287",
      "dimentio"
    ],
    [
      "5e4979d4fad3ca55f6512458",
      "zudo"
    ],
    [
      "63ce0c242c32d06d26b83d88",
      "fleower"
    ],
    [
      "5f1f3edf2b46ec11484cd359",
      "nez"
    ],
    [
      "640de57eaa4882e7599d9279",
      "toasty"
    ],
    [
      "629ebdc01de244ce13e5749c",
      "atomosu"
    ],
    [
      "5f3b3b25fdafc22e7e010648",
      "kawfi"
    ],
    [
      "5f1f3edf2b46ec11484cd359",
      "nez"
    ],
    [
      "5e88d0ead351fa71316ba29e",
      "promooooooo"
    ],
    [
      "62a51397bf6ce5ce2e84b6cd",
      "westl"
    ],
    [
      "5ebc6e7e0cfca96246a9a274",
      "ninetales"
    ],
    [
      "5e844b0868270e617d52c990",
      "czsmall0402"
    ],
    [
      "5fb0b7183a6a8b21b897e660",
      "kezdabez"
    ],
    [
      "5f5dbcc4454e942b4fdfc5fa",
      "vincehd"
    ],
    [
      "5e9c3ae0530bc13f9fdcbf1e",
      "flower"
    ],
    [
      "5eea0ff69a1ba76c20347086",
      "furry"
    ],
    [
      "647a4d2a7410533f722272dc",
      "takathebot"
    ],
    [
      "5eeaa5535b5c156c224f5265",
      "deep4amthoughts"
    ],
    [
      "5e7679c65bc0c31aba22746d",
      "doktorocelot"
    ],
    [
      "6049b70b3e33d5b0d2398f53",
      "flowerpetal"
    ],
    [
      "5e41118d4be6907ad7eab8b2",
      "matthew"
    ],
    [
      "5e49a95efad3ca55f6512742",
      "zaptor"
    ],
    [
      "5e40866a4be6907ad7eab093",
      "blink"
    ],
    [
      "5e3324299380f13edda2b1b1",
      "flash"
    ],
    [
      "5e32fe50d3e20d2bd2bf234f",
      "gebruikersnaam"
    ],
    [
      "5e3361351da5014141c84788",
      "thugginator"
    ],
    [
      "60cc700a866c6a2d2c8740a7",
      "sketchedpurple"
    ],
    [
      "5e3d85b42445dd5207770d0c",
      "garbo"
    ],
    [
      "5e34855fc8035a4c0f43df4a",
      "glitchypsi"
    ],
    [
      "5e342a67f3cfa44539619a85",
      "nook"
    ],
    [
      "6189eb0e842d86b339edc62e",
      "sen"
    ],
    [
      "5ec588fb26fbfd625b827203",
      "rinse"
    ],
    [
      "6098518e3d5155e6ec429cdc",
      "dan63"
    ],
    [
      "5e769ea53b02231ab3bc5a06",
      "tanarus"
    ],
    [
      "60c9db348b04692d0c4c5684",
      "meow"
    ],
    [
      "611444f95db7755ac3281996",
      "dragon"
    ],
    [
      "5e57272541d15b6621450666",
      "sinewave"
    ],
    [
      "5e76729135486f1ac1e6e163",
      "blizz"
    ],
    [
      "5e6500b192b7d95060a05765",
      "brendel"
    ],
    [
      "61b8f799381c530380da0f1f",
      "3621"
    ],
    [
      "60eb27d9d4d84832cce13ce1",
      "barho"
    ],
    [
      "5e82482d552a596c95f408fd",
      "stepper"
    ],
    [
      "5f81be091404653c00f8eb8e",
      "littlewolf"
    ],
    [
      "6529e05a39e055b505a04e4b",
      "fountain"
    ],
    [
      "609b0c9ccea7231a8a9a0da7",
      "iika"
    ]
  ];

pub async fn run_for_all_users<ReturnType: Clone, U: Future<Output = Result<Packet<ReturnType>, InMemoryReqwestError>>, T: FnMut(&'static str) -> U>(mut func: T) -> Vec<Packet<ReturnType>> {
    let users = join_all(TETRIO_USERS.iter().map(|s| func(s[0]))).await;

    users.iter().enumerate().filter_map(|(index, user)|  {
        let username = TETRIO_USERS[index][1];
        match user {
            Ok(value) => {
                match value {
                    Packet {cache: Some(_cache), data: Some(_data), success: true, error: None} => {
                        Some(value.clone())
                    },
                    Packet {error: Some(error), ..} => {
                        eprintln!("An error occured for user {}: {}", username, error.msg);
                        None
                    },
                    _ => None
                }
            },
            Err(err) => {
                eprintln!("An error occured for user {}: {err:?}", username);
                None
            }
        }
    }).collect::<Vec<_>>()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = InMemoryReqwestClient::default();

    // let user_info_packets = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}"), None)).await;



    // let ids = user_info_packets.iter().map(|packet| {
    //      let user = packet.data.as_ref().unwrap();
    //     return [user.get("_id").cloned().unwrap().as_str().map(|v| v.to_string()).unwrap(), user.get("username").cloned().unwrap().as_str().map(|v| v.to_string()).unwrap()]
    // }).collect::<Vec<_>>();

    // let user_info_packets = user_infos.iter().map(|packet| {
    //     return packet
    // }).collect::<Vec<_>>();

    // fs::write("./tetrio_users_ids.json", serde_json::to_string_pretty(&ids)?)?;
    // fs::write("./tetrio_users_infos.json", serde_json::to_string_pretty(&user_info_packets)?)?;

    // User Summaries
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries"), None)).await;
    // fs::write("./tetrio_users_all_summaries.json", serde_json::to_string_pretty(&data)?)?;
    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/40l"), None)).await;
    // fs::write("./tetrio_users_40l_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/blitz"), None)).await;
    // fs::write("./tetrio_users_blitz_summaries.json", serde_json::to_string_pretty(&data)?)?;
    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/zenith"), None)).await;
    // fs::write("./tetrio_users_zenith_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/zenithex"), None)).await;
    // fs::write("./tetrio_users_zenithex_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/league"), None)).await;
    // fs::write("./tetrio_users_league_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/zen"), None)).await;
    // fs::write("./tetrio_users_zen_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/summaries/achievements"), None)).await;
    // fs::write("./tetrio_users_achievements_summaries.json", serde_json::to_string_pretty(&data)?)?;

    // // User recent personal records    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/40l/recent"), None)).await;
    // fs::write("./tetrio_users_40l_records_recent.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/blitz/recent"), None)).await;
    // fs::write("./tetrio_users_blitz_records_recent.json", serde_json::to_string_pretty(&data)?)?;
    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenith/recent"), None)).await;
    // fs::write("./tetrio_users_zenith_records_recent.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenithex/recent"), None)).await;
    // fs::write("./tetrio_users_zenithex_records_recent.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/league/recent"), None)).await;
    // fs::write("./tetrio_users_league_records_recent.json", serde_json::to_string_pretty(&data)?)?;

    // // User top personal records    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/40l/top"), None)).await;
    // fs::write("./tetrio_users_40l_records_top.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/blitz/top"), None)).await;
    // fs::write("./tetrio_users_blitz_records_top.json", serde_json::to_string_pretty(&data)?)?;
    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenith/top"), None)).await;
    // fs::write("./tetrio_users_zenith_records_top.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenithex/top"), None)).await;
    // fs::write("./tetrio_users_zenithex_records_top.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/league/top"), None)).await;
    // fs::write("./tetrio_users_league_records_top.json", serde_json::to_string_pretty(&data)?)?;

    // // User top personal records    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/40l/progression"), None)).await;
    // fs::write("./tetrio_users_40l_records_progression.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/blitz/progression"), None)).await;
    // fs::write("./tetrio_users_blitz_records_progression.json", serde_json::to_string_pretty(&data)?)?;
    
    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenith/progression"), None)).await;
    // fs::write("./tetrio_users_zenith_records_progression.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/zenithex/progression"), None)).await;
    // fs::write("./tetrio_users_zenithex_records_progression.json", serde_json::to_string_pretty(&data)?)?;

    // let data = run_for_all_users(|user| client.make_tetrio_api_request::<serde_json::Value>(format!("users/{user}/records/league/progression"), None)).await;
    // fs::write("./tetrio_users_league_records_progression.json", serde_json::to_string_pretty(&data)?)?;

    let mut query = ValueBoundQuery::NotBound { limit: Some(100), country: None, };
    let session_id = "X_TEST_BEAN_BLASTER";
    let url = format!("users/by/{}", "league");
    let result = client.make_tetrio_api_request::<serde_json::Value>(CachedClient::<ReqwestClient, MokaCache>::make_url(&url, &query.as_query_params()), Some(session_id)).await?;
    let result = result.data.unwrap_or(json! ({ 
      "entries": []
    }));
    let default_p = json!( Prisecter {pri: 0., sec: 0., ter: 0. } );
    let entries = result.get("entries").ok_or("Couldn't fetch leaderboard!")?.as_array().ok_or("Couldn't fetch leaderboard!")?;
    let p = entries.last().map(|entry| serde_json::from_value::<Prisecter>(entry.get("p").unwrap_or(&default_p).clone())).ok_or("Couldn't fetch leaderboard!")??;

    let mut results = vec![];
    results.push(entries.clone());

    query = ValueBoundQuery::After { after: p, limit: Some(100), country: None };
    loop {
      
      let session_id = "X_BEANBLASTER";
      let url = format!("users/by/{}", "league");
      let result = client.make_tetrio_api_request::<serde_json::Value>(CachedClient::<ReqwestClient, MokaCache>::make_url(&url, &query.as_query_params()), Some(session_id)).await?;
      let result = result.data.unwrap_or(json! ({ 
        "entries": []
      }));
      let default_p = json!( Prisecter {pri: 0., sec: 0., ter: 0. } );
      let entries = result.get("entries").ok_or("Couldn't fetch leaderboard!")?.as_array().ok_or("Couldn't fetch leaderboard!")?;
      
      let p = entries.last().map(|entry| serde_json::from_value::<Prisecter>(entry.get("p").unwrap_or(&default_p).clone())).ok_or("Couldn't fetch leaderboard!")??;
     
      query = ValueBoundQuery::After { after: p, limit: Some(100), country: None };
  
      results.push(entries.clone());


      if entries.len() != 100 {
        break;
      }
    }
    fs::write("./tetrio_full_league_leaderboard.json", serde_json::to_string_pretty(&results)?)?;

    Ok(())
}