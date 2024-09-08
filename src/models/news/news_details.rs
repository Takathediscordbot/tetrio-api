use serde::{Deserialize, Serialize};
use crate::models::{common::APIstring, packet::Packet};

#[derive(Serialize, Deserialize, Clone)]
pub struct News {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub stream: APIstring,
    #[serde(rename = "type")]
    pub item_type: APIstring,
    pub data: serde_json::Value,
    pub ts: APIstring
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewsPacketData {
    pub news: Vec<News>
}

pub type NewsPacket = Packet<NewsPacketData>;
