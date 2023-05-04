use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConnection {
    pub id: String,
    pub username: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConnections {
    pub discord: Option<DiscordConnection>
}