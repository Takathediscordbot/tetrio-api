//! [Latest News](https://tetr.io/about/api/#newsstream) models
//! As the documentation for those models is not available, the fields are not documented.
//! You will have to test the routes yourself to figure out which fields you can get.


use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;

#[derive(Serialize, Deserialize, Clone)]
pub struct News {
    #[serde(rename = "_id")]
    /// The item's internal ID.
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
pub struct NewsPacketData {
    /// The latest news items:
    pub news: Vec<News>
}

/// The packet returned by the API when requesting the latest news.
pub type NewsPacket = Packet<NewsPacketData>;