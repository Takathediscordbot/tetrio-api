
use serde::{Deserialize, Serialize};

use crate::{http::parameters::value_bound_query::Prisecter, models::{common::{APIfloat, APIint, APIstring}, packet::Packet}};

use super::{user_achievements::UserArCounts, user_rank::UserRank, user_role::UserRole};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeaderboardUserLeague {
    /* The amount of TETRA LEAGUE games played by this user. */
    pub gamesplayed: APIint,
    /* The amount of TETRA LEAGUE games won by this user. */
    pub gameswon: APIint,
    /* This user's TR (Tetra Rating). */
    pub tr: APIfloat,
    /* This user's GLIXARE. */
    pub gxe: APIfloat,
    /* This user's letter rank. */
    pub rank: Option<UserRank>,
    /* This user's highest achieved rank this season. */
    pub bestrank: Option<UserRank>,
    /* This user's Glicko-2 rating. */
    pub glicko: APIfloat,
    /* This user's Glicko-2 Rating Deviation. */
    pub rd: APIfloat,
    /* This user's average APM (attack per minute) over the last 10 games. */
    pub apm: APIfloat,
    /* This user's average PPS (pieces per second) over the last 10 games. */
    pub pps: APIfloat,
    /* This user's average VS (versus score) over the last 10 games. */
    pub vs: APIfloat,
    /* Whether this user's RD is rising (has not played in the last week). */
    pub decaying: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeaderboardUser {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub username: APIstring,
    pub role: UserRole,
    pub ts: Option<APIstring>,
    pub xp: APIfloat,
    pub country: Option<APIstring>,
    pub supporter: Option<bool>,
    pub league: LeaderboardUserLeague,
    pub gamesplayed: APIint,
    pub gameswon: APIint,
    pub gametime: APIfloat,
    pub friend_count: Option<APIint>,
    pub ar: APIint,
    pub ar_counts: UserArCounts,
    pub p: Prisecter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeaderboardObject {
    pub entries: Vec<LeaderboardUser>
}

pub type LeaderboardPacket = Packet<LeaderboardObject>;
