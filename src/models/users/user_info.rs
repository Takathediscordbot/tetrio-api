//! 
//![User Info](https://tetr.io/about/api/#usersuser) models 

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use crate::models::packet::Packet;
use crate::models::users::user_badge::UserBadge;
use crate::models::users::user_connections::UserConnections;
use crate::models::users::user_distinguishment::UserDistinguishment;
use crate::models::users::user_rank::UserRank;
use crate::models::users::user_role::UserRole;


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// This user's current TETRA LEAGUE standing:
pub struct UserInfoLeague {
    /// The amount of TETRA LEAGUE games played by this user.
    pub gamesplayed: i64,
    /// The amount of TETRA LEAGUE games won by this user.
    pub gameswon: i64,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub rating: f64,
    /// This user's letter rank. Z is unranked.
    pub rank: UserRank,
    /// This user's highest achieved rank this season.
    pub bestrank: Option<UserRank>,
    /// This user's position in global leaderboards, or -1 if not applicable.
    pub standing: i64,
    /// This user's position in local leaderboards, or -1 if not applicable.
    pub standing_local: i64,
    /// The next rank this user can achieve, if they win more games, or null if unranked (or the best rank).
    pub next_rank: Option<UserRank>,
    /// The previous rank this user can achieve, if they lose more games, or null if unranked (or the worst rank).
    pub prev_rank: Option<UserRank>,
    /// The position of the best player in the user's current rank, surpass them to go up a rank. -1 if unranked (or the best rank).
    pub next_at: i64,
    /// The position of the worst player in the user's current rank, dip below them to go down a rank. -1 if unranked (or the worst rank).
    pub prev_at: i64,
    /// This user's percentile position (0 is best, 1 is worst).
    pub percentile: f64,
    /// This user's percentile rank, or Z if not applicable.
    pub percentile_rank: UserRank,
    /// This user's Glicko-2 rating.
    pub glicko: Option<f64>,
    /// This user's Glicko-2 Rating Deviation. If over 100, this user is unranked.
    pub rd: Option<f64>,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    pub decaying: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfoUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    /// The user's username.
    pub username: Arc<str>,
    /// The user's role (one of "anon", "user", "bot", "halfmod", "mod", "admin", "sysop", "banned").
    pub role: UserRole,
    /// When the user account was created. If not set, this account was created before join dates were recorded.
    pub ts: Option<Arc<str>>,
    /// If this user is a bot, the bot's operator.
    pub botmaster: Option<Arc<str>>,
    /// The user's badges
    pub badges: Box<[UserBadge]>,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gamesplayed: i64,
    /// The amount of online games won by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gameswon: i64,
    /// The amount of seconds this user spent playing, both on- and offline. If the user has chosen to hide this statistic, it will be -1.
    pub gametime: f64,
    /// The user's ISO 3166-1 country code, or null if hidden/unknown. Some vanity flags exist.
    pub country: Option<Arc<str>>,
    /// Whether this user currently has a bad standing (recently banned).
    pub badstanding: Option<bool>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>,
    /// An indicator of their total amount supported, between 0 and 4 inclusive.
    pub supporter_tier: i64,
    /// Whether this user is a verified account.
    pub verified: bool,
    /// This user's current TETRA LEAGUE standing:
    pub league: UserInfoLeague,
    /// This user's avatar ID. Get their avatar at https://tetr.io/user-content/avatars/{ USERID }.jpg?rv={ AVATAR_REVISION }
    pub avatar_revision: Option<i64>,
    /// This user's banner ID. Get their banner at https://tetr.io/user-content/banners/{ USERID }.jpg?rv={ BANNER_REVISION }. Ignore this field if the user is not a supporter.
    pub banner_revision: Option<i64>,
    /// This user's "About Me" section. Ignore this field if the user is not a supporter.
    pub bio: Option<Arc<str>>,
    /// This user's third party connections:
    pub connections: UserConnections,
    /// The amount of players who have added this user to their friends list.
    pub friend_count: Option<i64>,
    /// This user's distinguishment banner, if any. Must at least have: type (string)
    pub distinguishment: Option<UserDistinguishment>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoPacketData {
    /// The requested user
    pub user: UserInfoUser
}

/// A packet containing a user's info
pub type UserInfoPacket = Packet<UserInfoPacketData>;


