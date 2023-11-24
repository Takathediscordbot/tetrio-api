use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;

#[derive(Serialize, Deserialize, Clone)]
pub struct News {
    #[serde(rename = "_id")]
    pub id: String,
    pub stream: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub data: serde_json::Value,
    pub ts: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewsPacketData {
    pub news: Vec<News>
}

pub type NewsPacket = Packet<NewsPacketData>;