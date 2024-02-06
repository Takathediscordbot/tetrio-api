use std::sync::Arc;

use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserSearch {
    /// The TETRIO user id
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    /// The DISCORD username
    pub username: Arc<str>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserSearchPacketData {
    /// The user data
    pub user: UserSearch,
}

pub type UserSearchPacket = Packet<UserSearchPacketData>;
