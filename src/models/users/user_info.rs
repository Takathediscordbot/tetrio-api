use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;
use crate::models::users::user_badge::UserBadge;
use crate::models::users::user_connections::UserConnections;
use crate::models::users::user_distinguishment::UserDistinguishment;
use crate::models::users::user_rank::UserRank;
use crate::models::users::user_role::UserRole;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfoLeague {
    pub gamesplayed: i64,
    pub gameswon: i64,
    pub rating: f64,
    pub rank: UserRank,
    pub bestrank: Option<UserRank>,
    pub standing: i64,
    pub standing_local: i64,
    pub next_rank: Option<UserRank>,
    pub prev_rank: Option<UserRank>,
    pub next_at: i64,
    pub prev_at: i64,
    pub percentile: f64,
    pub percentile_rank: UserRank,
    pub glicko: Option<f64>,
    pub rd: Option<f64>,
    pub apm: Option<f64>,
    pub pps: Option<f64>,
    pub vs: Option<f64>,
    pub decaying: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfoUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub ts: Option<String>,
    pub botmaster: Option<String>,
    pub badges: Vec<UserBadge>,
    pub xp: f64,
    pub gamesplayed: i64,
    pub gameswon: i64,
    pub gametime: f64,
    pub country: Option<String>,
    pub badstanding: Option<bool>,
    pub supporter: Option<bool>,
    pub supporter_tier: i64,
    pub verified: bool,
    pub league: UserInfoLeague,
    pub avatar_revision: Option<i64>,
    pub banner_revision: Option<i64>,
    pub bio: Option<String>,
    pub connections: UserConnections,
    pub friend_count: Option<i64>,
    pub distinguishment: Option<UserDistinguishment>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoPacketData {
    pub user: UserInfoUser
}

pub type UserInfoPacket = Packet<UserInfoPacketData>;


