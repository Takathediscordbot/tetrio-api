/*

*/

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecordTime {
    start: i64,
    zero: bool,
    locked: bool,
    prev: i64,
    frameoffset: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzClears {
    singles: i16,
    doubles: i16,
    triples: i16,
    quads: i16,
    pentas: Option<i16>,
    realtspins: i16,
    minitspins: i16,
    minitspinsingles: i16,
    tspinsingles: i16,
    minitspindoubles: i16,
    tspindoubles: i16,
    tspintriples: i16,
    tspinquads: i16,
    tspinpentas: Option<i16>,
    allclear: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzGarbage {
    sent: i16,
    received: i16,
    attack: i16,
    cleared: i16    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzFinesse {
    combo: i16,
    faults: i32,
    perfectpieces: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecordEndContext {
    seed: i64,
    lines: i16,
    level_lines: i16,
    level_lines_needed: i16,
    inputs: i16,
    holds: Option<i16>,
    time: BlitzRecordTime,
    score: i64,
    zenlevel: i16,
    zenprogress: i16,
    level: i16,
    combo: i16,
    currentcombopower: i16,
    topcombo: i16,
    btb: i16,
    topbtb: i16,
    currentbtbchainpower: Option<i16>,
    tspins: i16,
    piecesplaced: i16,
    clears: BlitzClears,
    garbage: BlitzGarbage,
    kills: i16,
    finesse: BlitzFinesse,
    #[serde(rename = "finalTime")]
    final_time: f64,
    gametype: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecordUser {
    _id: Arc<str>,
    username: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecordDetails {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub endcontext: BlitzRecordEndContext,
    #[serde(rename = "ismulti")]
    is_multi: Option<bool>,
    #[serde(rename = "replayid")]
    replay_id: Arc<str>,
    stream: Arc<str>,
    ts: Arc<str>,
    user: BlitzRecordUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzRecord {
    pub record: Option<BlitzRecordDetails>,
    pub rank: Option<i64>,
}
