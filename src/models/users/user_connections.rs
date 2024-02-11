use serde::{Deserialize, Serialize};

/// This user's connection to Discord:
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscordConnection {
    /// This user's Discord ID.
    pub id: String,
    /// This user's Discord Tag.
    pub username: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// This user's third party connections:
pub struct UserConnections {
    /// This user's connection to Discord:
    pub discord: Option<DiscordConnection>
}