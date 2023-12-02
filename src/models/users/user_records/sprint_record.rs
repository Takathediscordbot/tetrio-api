/*

*/

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecordTime {
    start: i64,
    zero: bool,
    locked: bool,
    prev: i64,
    frameoffset: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintClears {
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
pub struct SprintGarbage {
    sent: i16,
    received: i16,
    attack: i16,
    cleared: i16    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintFinesse {
    combo: i16,
    faults: i32,
    perfectpieces: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecordEndContext {
    seed: i64,
    lines: i16,
    level_lines: i16,
    level_lines_needed: i16,
    inputs: i16,
    holds: Option<i16>,
    time: SprintRecordTime,
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
    clears: SprintClears,
    garbage: SprintGarbage,
    kills: i16,
    finesse: SprintFinesse,
    #[serde(rename = "finalTime")]
    final_time: f64,
    gametype: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecordUser {
    _id: Arc<str>,
    username: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecordDetails {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub endcontext: SprintRecordEndContext,
    #[serde(rename = "ismulti")]
    is_multi: Option<bool>,
    #[serde(rename = "replayid")]
    replay_id: Arc<str>,
    stream: Arc<str>,
    ts: Arc<str>,
    user: SprintRecordUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintRecord {
    pub record: Option<SprintRecordDetails>,
    pub rank: Option<i64>,
}
