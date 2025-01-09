use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoutubeConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SteamConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConnections {
    pub discord: Option<DiscordConnection>,
    pub twitch: Option<TwitchConnection>,
    pub twitter: Option<TwitterConnection>,
    pub reddit: Option<RedditConnection>,
    pub youtube: Option<YoutubeConnection>,
    pub steam: Option<SteamConnection>,
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}
