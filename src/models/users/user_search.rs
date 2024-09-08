use crate::models::{common::APIstring, packet::Packet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSearch {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub username: APIstring,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSearchPacketData {
    pub user: UserSearch,
}

pub type UserSearchPacket = Packet<UserSearchPacketData>;
