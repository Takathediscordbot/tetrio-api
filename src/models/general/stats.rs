use crate::models::{common::{APIfloat, APIint}, packet::Packet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    pub usercount: APIint,
    pub usercount_delta: APIfloat,
    pub anoncount: APIint,
    pub totalaccounts: APIint,
    pub rankedcount: APIint,
    pub recordcount: APIint,
    pub gamesplayed: APIint,
    pub gamesplayed_delta: APIfloat,
    pub gamesfinished: APIint,
    pub gametime: APIfloat,
    pub inputs: APIint,
    pub piecesplaced: APIint,
}

pub type StatsPacket = Packet<Stats>;
