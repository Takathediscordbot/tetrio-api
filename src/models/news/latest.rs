use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNews {
    #[serde(rename = "_id")]
    pub id: String,
    pub stream: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub data: serde_json::Value,
    pub ts: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNewsPacketData {
    pub news: Vec<LatestNews>
}

pub type LatestNewsPacket = Packet<LatestNewsPacketData>;