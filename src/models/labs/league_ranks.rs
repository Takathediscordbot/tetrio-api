use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{
    common::{APIfloat, APIint, APIstring},
    packet::Packet,
    users::user_rank::UserRank,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueRank {
    /* The leaderboard position required to attain this rank. */
    pub pos: APIint,
    /* The percentile (0~1) this rank is for. */
    pub percentile: APIfloat,
    /* The TR required to obtain a leaderboard position that will award this rank. */
    pub tr: APIfloat,
    /* The TR this rank will gravitate toward (using de- and inflation zones). */
    pub targettr: APIfloat,
    /* The average APM across all players in this rank. */
    pub apm: Option<APIfloat>,
    /* The average PPS across all players in this rank. */
    pub pps: Option<APIfloat>,
    /* The average Versus Score across all players in this rank. */
    pub vs: Option<APIfloat>,
    /* The amount of players with this rank. */
    pub count: APIint,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueRanksData {
    pub total: APIint,
    #[serde(flatten)]
    pub ranks: HashMap<UserRank, LeagueRank>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueRanks {
    /* The internal ID of the Labs data point. */
    pub _id: APIstring,
    /* The stream ID the Labs data point belongs to. */
    pub s: APIstring,
    /* The time at which the data point was created. */
    pub t: APIstring,
    /* The data point: */
    pub data: LeagueRanksData,
}

pub type LeagueRanksPacket = Packet<LeagueRanks>;
