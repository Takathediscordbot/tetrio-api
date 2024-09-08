
use serde::{Serialize, Deserialize};

use crate::models::{common::{APIfloat, APIint, APIstring}, packet::Packet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextUser {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub username: APIstring
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextHandling {
    pub arr: APIfloat,
    pub das: APIfloat,
    pub dcd: APIfloat,
    pub sdf: APIfloat,
    pub safelock: bool,
    pub cancel: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtra {
    pub vs: APIfloat
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPointExtraTracking {
    #[serde(rename = "aggregatestats___vsscore")]
    pub aggregate_stats_vs_score: Vec<APIfloat>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContextPoints {
    pub primary: APIint,
    pub secondary: APIfloat,
    pub tertiary: APIfloat,
    pub extra: LeagueEndContextPointExtra,
    #[serde(rename = "secondaryAvgTracking")]
    pub secondary_avg_tracking: Vec<APIfloat>,
    #[serde(rename = "tertiaryAvgTracking")]
    pub tertiary_avg_tracking: Vec<APIfloat>,
    #[serde(rename = "extraAvgTracking")]
    pub extra_avg_tracking: LeagueEndContextPointExtraTracking,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueEndContext {
    pub user: Option<LeagueEndContextUser>,

    pub id: Option<APIstring>,
    pub username: Option<APIstring>,
    pub handling: LeagueEndContextHandling,
    pub active: bool,
    pub success: bool,
    pub inputs: APIint,
    #[serde(rename = "piecesplaced")]
    pub pieces_placed: APIint,
    #[serde(rename = "naturalorder")]
    pub natural_order: APIfloat,
    pub score: APIint,
    pub wins: APIint,
    pub points: LeagueEndContextPoints
}

impl LeagueEndContext {
    pub fn get_id(&self) -> Option<APIstring> {
        self.id.clone().or(self.user.clone().map(|user| user.id))
    }

    pub fn get_username(&self) -> Option<APIstring> {
        self.username.clone().or(self.user.clone().map(|user| user.username))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStreamUser {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub username: APIstring
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStreamRecords {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub endcontext: Vec<LeagueEndContext>,
    #[serde(rename = "ismulti")]
    pub is_multi: bool,
    #[serde(rename = "replayid")]
    pub replay_id: APIstring,
    pub stream: APIstring,
    pub ts: APIstring,
    pub user: LeagueStreamUser
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueStream {
    pub records: Vec<LeagueStreamRecords>
}

pub type LeagueStreamPacket = Packet<LeagueStream>;
