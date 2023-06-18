use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSearch {
    #[serde(rename = "_id")]
    pub id: Box<str>,
    pub username: Box<str>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSearchPacketData {
    pub user: UserSearch,
}

pub type UserSearchPacket = Packet<UserSearchPacketData>;
