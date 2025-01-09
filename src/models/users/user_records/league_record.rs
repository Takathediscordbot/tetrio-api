/*

*/


use std::collections::HashMap;

use serde::{Deserialize, Serialize};


use crate::{http::parameters::{personal_user_records::GameMode, value_bound_query::Prisecter}, models::{common::{APIArray, APIfloat, APIint, APIsmallint, APIstring}, users::user_rank::UserRank}};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueLeaderboardStats {
    pub apm: Option<APIfloat>,
    pub pps: Option<APIfloat>,
    pub vsscore: Option<APIfloat>,
    pub garbagesent: APIfloat,
    pub garbagereceived: APIfloat,
    pub kills: APIfloat,
    pub altitude: APIfloat,
    pub rank: APIfloat,
    pub targetingfactor: APIfloat,
    pub targetinggrace: APIfloat,
    #[serde(rename="revivesMaxOfBoth")]
    pub revives_max_of_both: Option<APIint>
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct ShadowedBy {
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Shadows {
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueLeaderboard {
    /* The player's User ID. */
    pub id: APIstring,
    /* The player's username. */
    pub username: APIstring,
    /* Whether the player is still in the game. If false, the user has likely been disqualified. */
    pub active: bool,
    /* The amount of rounds won by the player. */
    pub wins: APIint,
    /* The aggregate stats across all rounds. */
    pub stats: LeagueLeaderboardStats,

    pub naturalorder: Option<APIsmallint>,

    #[serde(rename = "shadowedBy")]
    pub shadowed_by: Vec<Option<ShadowedBy>>,

    pub shadows: Vec<Option<Shadows>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueRoundStats {
    pub apm: APIfloat,
    pub pps: APIfloat,
    pub vsscore: APIfloat,
    pub garbagesent: APIfloat,
    pub garbagereceived: APIfloat,
    pub kills: APIfloat,
    pub altitude: APIfloat,
    pub rank: APIfloat,
    pub targetingfactor: APIfloat,
    pub targetinggrace: APIfloat,
    #[serde(rename="revivesMaxOfBoth")]
    pub revives_max_of_both: Option<APIint>
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueRound {
    /* The player's User ID. */
    pub id: APIstring,
    /* The player's username. */
    pub username: APIstring,
    /* Whether the player is still in the game. If false, the user has likely been disqualified for the round. */
    pub active: bool,
    /* Whether the player made it through the round alive. */
    pub alive: bool,
    /* The time alive in this match. */
    pub lifetime: APIint,
    /* The aggregate stats for the player for this round. */
    pub stats: LeagueRoundStats,
    pub naturalorder: Option<APIsmallint>,

    #[serde(rename = "shadowedBy")]
    pub shadowed_by: Vec<Option<ShadowedBy>>,

    pub shadows: Vec<Option<Shadows>>

}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueResults {
    /* The final leaderboard at the end of the match */
    pub leaderboard: APIArray<LeagueLeaderboard>,
    /* The scoreboards for every round */
    pub rounds: APIArray<APIArray<LeagueRound>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueRecordUser {
    pub id: APIstring,
    pub username: APIstring,
    pub avatar_revision: Option<APIint>,
    pub banner_revision: Option<APIint>,
    pub country: Option<APIstring>,
    pub supporter: bool
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeagueExtrasData {
    pub glicko: APIfloat,
    pub placement: Option<APIfloat>,
    pub rank: UserRank,
    pub rd: APIfloat,
    pub tr: APIfloat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeagueExtras {
    pub league: HashMap<APIstring, Vec<Option<LeagueExtrasData>>>,
    pub result: APIstring
}


#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct LeagueRecord {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub replayid: APIstring,
    pub stub: bool,
    pub gamemode: GameMode,
    pub pb: bool,
    pub oncepb: bool,
    pub ts: APIstring,
    pub revolution: Option<APIstring>,
    pub user: Option<LeagueRecordUser>,
    pub otherusers: Vec<LeagueRecordUser>,
    pub leaderboards: Vec<APIstring>,
    pub results: LeagueResults,
    pub extras: LeagueExtras,
    pub disputed: bool,
    pub p: Prisecter
}
