use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoutubeConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SteamConnection {
    pub id: APIstring,
    pub username: APIstring,
    pub display_username: APIstring
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConnections {
    pub discord: Option<DiscordConnection>,
    pub twitch: Option<TwitchConnection>,
    pub twitter: Option<TwitterConnection>,
    pub reddit: Option<RedditConnection>,
    pub youtube: Option<YoutubeConnection>,
    pub steam: Option<SteamConnection>
}
