use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextUser {
    #[serde(rename = "_id")]
    id: String,
    username: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextHandling {
    arr: f64,
    das: f64,
    dcd: f64,
    sdf: f64,
    safelock: bool,
    cancel: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtra {
    vs: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtraTracking {
    #[serde(rename = "aggregatestats___vsscore")]
    aggregate_stats_vs_score: Vec<f64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPoints {
    primary: i32,
    secondary: f64,
    tertiary: f64,
    extra: LeagueEndContextPointExtra,
    #[serde(rename = "secondaryAvgTracking")]
    secondary_avg_tracking: Vec<f64>,
    #[serde(rename = "tertiaryAvgTracking")]
    tertiary_avg_tracking: Vec<f64>,
    #[serde(rename = "extraAvgTracking")]
    extra_avg_tracking: LeagueEndContextPointExtraTracking,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContext {
    user: LeagueEndContextUser,
    handling: LeagueEndContextHandling,
    active: bool,
    success: bool,
    inputs: i32,
    #[serde(rename = "piecesplaced")]
    pieces_placed: i32,
    #[serde(rename = "naturalorder")]
    natural_order: f32,
    score: i32,
    wins: i32,
    points: LeagueEndContextPoints
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LeagueStreamUser {
    #[serde(rename = "_id")]
    id: String,
    username: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStreamRecords {
    #[serde(rename = "_id")]
    id: String,
    endcontext: Vec<LeagueEndContext>,
    #[serde(rename = "ismulti")]
    is_multi: bool,
    #[serde(rename = "replayid")]
    replay_id: String,
    stream: String,
    ts: String,
    user: LeagueStreamUser
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStream {
    records: Vec<LeagueStreamRecords>
}