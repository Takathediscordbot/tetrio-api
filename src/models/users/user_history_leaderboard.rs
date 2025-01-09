use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{http::parameters::value_bound_query::Prisecter, models::{common::{APIfloat, APIint, APIstring}, packet::Packet}};

use super::user_rank::UserRank;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LeaderboardUser {
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    /* The user's internal ID. */
    pub _id: APIstring,
    /* The season ID. */
    pub season: APIstring,
    /* The username the user had at the time. */
    pub username: APIstring,
    /* The country the user represented at the time. */
    pub country: Option<APIstring>,
    /* This user's final position in the season's global leaderboards. */
    pub placement: APIint,
    /* Whether the user was ranked at the time of the season's end. */
    pub ranked: bool,
    /* The amount of TETRA LEAGUE games played by this user. */
    pub gamesplayed: APIint,
    /* The amount of TETRA LEAGUE games won by this user. */
    pub gameswon: APIint,
    /* This user's final Glicko-2 rating. */
    pub glicko: APIfloat,
    /* This user's final Glicko-2 Rating Deviation. */
    pub rd: APIfloat,
    /* This user's final TR (Tetra Rating). */
    pub tr: APIfloat,
    /* This user's final GLIXARE score (a % chance of beating an average player). */
    pub gxe: APIfloat,
    /* This user's final letter rank. */
    pub rank: UserRank,
    /* This user's highest achieved rank in the season. */
    pub bestrank: Option<UserRank>,
    /* This user's average APM (attack per minute) over the last 10 games in the season. */
    pub apm: APIfloat,
    /* This user's average PPS (pieces per second) over the last 10 games in the season. */
    pub pps: APIfloat,
    /* This user's average VS (versus score) over the last 10 games in the season. */
    pub vs: APIfloat,
    /* The prisecter of this entry: */
    pub p: Prisecter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHistoryLeaderboard {
    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub entries: Vec<LeaderboardUser>
}

pub type HistoricalLeaderboardPacket = Packet<UserHistoryLeaderboard>;

