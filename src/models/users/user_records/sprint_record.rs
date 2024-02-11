/*

*/

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintRecordTime {
    pub start: i64,
    pub zero: bool,
    pub locked: bool,
    pub prev: i64,
    pub frameoffset: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintClears {
    pub singles: i16,
    pub doubles: i16,
    pub triples: i16,
    pub quads: i16,
    pub pentas: Option<i16>,
    pub realtspins: i16,
    pub minitspins: i16,
    pub minitspinsingles: i16,
    pub tspinsingles: i16,
    pub minitspindoubles: i16,
    pub tspindoubles: i16,
    pub tspintriples: i16,
    pub tspinquads: i16,
    pub tspinpentas: Option<i16>,
    pub allclear: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintGarbage {
    pub sent: i16,
    pub received: i16,
    pub attack: Option<i16>,
    pub cleared: i16    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintFinesse {
    pub combo: i16,
    pub faults: i32,
    pub perfectpieces: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintRecordEndContext {
    pub seed: i64,
    pub lines: i16,
    pub level_lines: i16,
    pub level_lines_needed: i16,
    pub inputs: i16,
    pub holds: Option<i16>,
    pub time: SprintRecordTime,
    pub score: i64,
    pub zenlevel: i16,
    pub zenprogress: i16,
    pub level: i16,
    pub combo: i16,
    pub currentcombopower: Option<i16>,
    pub topcombo: i16,
    pub btb: i16,
    pub topbtb: i16,
    pub currentbtbchainpower: Option<i16>,
    pub tspins: i16,
    pub piecesplaced: i16,
    pub clears: SprintClears,
    pub garbage: SprintGarbage,
    pub kills: i16,
    pub finesse: SprintFinesse,
    #[serde(rename = "finalTime")]
    pub final_time: f64,
    pub gametype: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintRecordUser {
    pub _id: Arc<str>,
    pub username: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintRecordDetails {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub endcontext: SprintRecordEndContext,
    #[serde(rename = "ismulti")]
    pub is_multi: Option<bool>,
    #[serde(rename = "replayid")]
    pub replay_id: Arc<str>,
    pub stream: Arc<str>,
    pub ts: Arc<str>,
    pub user: SprintRecordUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintRecord {
    pub record: Option<SprintRecordDetails>,
    pub rank: Option<i64>,
}
