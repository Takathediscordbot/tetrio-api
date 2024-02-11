use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNews {
    /// The item's internal ID.
    #[serde(rename = "_id")]
    pub id: String,
    /// The item's stream.
    pub stream: String,
    #[serde(rename = "type")]
    /// The item's type.
    pub item_type: String,
    /// The item's records.
    pub data: serde_json::Value,
    ///  The item's creation date.
    pub ts: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestNewsPacketData {
    /// The latest news items:
    pub news: Vec<LatestNews>
}

/// The packet returned by the API when requesting the latest news.
pub type LatestNewsPacket = Packet<LatestNewsPacketData>;