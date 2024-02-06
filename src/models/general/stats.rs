use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    pub usercount: i64,
    pub usercount_delta: f64,
    pub anoncount: i64,
    pub rankedcount: i64,
    pub replaycount: i64,
    pub gamesplayed: i64,
    pub gamesplayed_delta: f64,
    pub gamesfinished: i64,
    pub gametime: f64,
    pub inputs: i64,
    pub piecesplaced: i64,
    pub totalaccounts: i64,
}

pub type StatsPacket = Packet<Stats>;
