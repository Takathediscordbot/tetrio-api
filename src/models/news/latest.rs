use serde::{Deserialize, Serialize};
use crate::models::{common::APIstring, packet::Packet};

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNews {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub stream: APIstring,
    #[serde(rename = "type")]
    pub item_type: APIstring,
    pub data: serde_json::Value,
    pub ts: APIstring
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNewsPacketData {
    pub news: Vec<LatestNews>
}

pub type LatestNewsPacket = Packet<LatestNewsPacketData>;
