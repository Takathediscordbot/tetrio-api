//!
//! Models used to represent a Blitz record of a user.
//! None of those models are documented in the API documentation
//! They are still tested but I can not provide documentation for each of them.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecordTime {
    pub start: i64,
    pub zero: bool,
    pub locked: bool,
    pub prev: i64,
    pub frameoffset: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzClears {
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
pub struct BlitzGarbage {
    pub sent: i16,
    pub received: i16,
    pub attack: i16,
    pub cleared: i16    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzFinesse {
    pub combo: i16,
    pub faults: i32,
    pub perfectpieces: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecordEndContext {
    pub seed: i64,
    pub lines: i16,
    pub level_lines: i16,
    pub level_lines_needed: i16,
    pub inputs: i16,
    pub holds: Option<i16>,
    pub time: BlitzRecordTime,
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
    pub clears: BlitzClears,
    pub garbage: BlitzGarbage,
    pub kills: i16,
    pub finesse: BlitzFinesse,
    #[serde(rename = "finalTime")]
    pub final_time: f64,
    pub gametype: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecordUser {
    pub _id: Arc<str>,
    pub username: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecordDetails {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub endcontext: BlitzRecordEndContext,
    #[serde(rename = "ismulti")]
    pub is_multi: Option<bool>,
    #[serde(rename = "replayid")]
    pub replay_id: Arc<str>,
    pub stream: Arc<str>,
    pub ts: Arc<str>,
    pub user: BlitzRecordUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecord {
    pub record: Option<BlitzRecordDetails>,
    pub rank: Option<i64>,
}
