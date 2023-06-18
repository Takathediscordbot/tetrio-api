use serde::{Serialize, Deserialize};

use crate::models::packet::Packet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextHandling {
    pub arr: f64,
    pub das: f64,
    pub dcd: f64,
    pub sdf: f64,
    pub safelock: bool,
    pub cancel: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtra {
    pub vs: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtraTracking {
    #[serde(rename = "aggregatestats___vsscore")]
    pub aggregate_stats_vs_score: Vec<f64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPoints {
    pub primary: i32,
    pub secondary: f64,
    pub tertiary: f64,
    pub extra: LeagueEndContextPointExtra,
    #[serde(rename = "secondaryAvgTracking")]
    pub secondary_avg_tracking: Vec<f64>,
    #[serde(rename = "tertiaryAvgTracking")]
    pub tertiary_avg_tracking: Vec<f64>,
    #[serde(rename = "extraAvgTracking")]
    pub extra_avg_tracking: LeagueEndContextPointExtraTracking,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContext {
    pub user: LeagueEndContextUser,
    pub handling: LeagueEndContextHandling,
    pub active: bool,
    pub success: bool,
    pub inputs: i32,
    #[serde(rename = "piecesplaced")]
    pub pieces_placed: i32,
    #[serde(rename = "naturalorder")]
    pub natural_order: f32,
    pub score: i32,
    pub wins: i32,
    pub points: LeagueEndContextPoints
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStreamUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStreamRecords {
    #[serde(rename = "_id")]
    pub id: String,
    pub endcontext: Vec<LeagueEndContext>,
    #[serde(rename = "ismulti")]
    pub is_multi: bool,
    #[serde(rename = "replayid")]
    pub replay_id: String,
    pub stream: String,
    pub ts: String,
    pub user: LeagueStreamUser
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStream {
    pub records: Vec<LeagueStreamRecords>
}

pub type LeagueStreamPacket = Packet<LeagueStream>;
