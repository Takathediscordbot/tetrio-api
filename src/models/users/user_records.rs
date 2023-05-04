use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecord {
    pub record: Option<serde_json::Value>,
    pub rank: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecord {
    pub record: Option<serde_json::Value>,
    pub rank: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRecordsInner {
    #[serde(rename = "40l")]
    pub sprint: SprintRecord,
    pub blitz: BlitzRecord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRecordsZen {
    pub level: i64,
    pub score: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRecords {
    pub records: UserRecordsInner,
    pub zen: UserRecordsZen,
}

pub type UserRecordsPacket = Packet<UserRecords>;
