
use serde::{Deserialize, Serialize};
use crate::models::common::{APIint, APIintarray, APIstring};
use crate::models::packet::Packet;
use crate::models::users::user_badge::UserBadge;
use crate::models::users::user_connections::UserConnections;
use crate::models::users::user_distinguishment::UserDistinguishment;
use crate::models::users::user_role::UserRole;

use super::user_achievements::UserArCounts;


// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct UserInfoLeague {
//     pub gamesplayed: APIint,
//     pub gameswon: APIint,
//     pub rating: f64,
//     pub rank: UserRank,
//     pub bestrank: Option<UserRank>,
//     pub standing: APIint,
//     pub standing_local: APIint,
//     pub next_rank: Option<UserRank>,
//     pub prev_rank: Option<UserRank>,
//     pub next_at: APIint,
//     pub prev_at: APIint,
//     pub percentile: f64,
//     pub percentile_rank: UserRank,
//     pub glicko: Option<f64>,
//     pub rd: Option<f64>,
//     pub apm: Option<f64>,
//     pub pps: Option<f64>,
//     pub vs: Option<f64>,
//     pub decaying: bool,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserInfo {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub username: APIstring,
    pub role: UserRole,
    pub ts: Option<APIstring>,
    pub botmaster: Option<APIstring>,
    pub badges: Box<[UserBadge]>,
    pub xp: f64,
    pub gamesplayed: APIint,
    pub gameswon: APIint,
    pub gametime: f64,
    pub country: Option<APIstring>,
    pub badstanding: Option<bool>,
    pub supporter: Option<bool>,
    pub supporter_tier: APIint,
    pub avatar_revision: Option<APIint>,
    pub banner_revision: Option<APIint>,
    pub bio: Option<APIstring>,
    pub connections: UserConnections,
    pub friend_count: Option<APIint>,
    pub distinguishment: Option<UserDistinguishment>,
    pub achievements: Box<APIintarray>,
    pub ar: APIint,
    pub ar_counts: UserArCounts
}


pub type UserInfoPacket = Packet<UserInfo>;


