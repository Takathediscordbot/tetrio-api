//!
//! [League Full Leaderboard](https://tetr.io/about/api/#userlistsleagueall) models

use std::sync::Arc;

use crate::models::packet::Packet;
use crate::models::users::user_rank::UserRank;
use crate::models::users::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// This user's current TETRA LEAGUE standing:
pub struct LeagueFullData {
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
#[serde(deny_unknown_fields)]
/// The matched user's data
pub struct LeagueFullUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    /// The user's username.
    pub username: Arc<str>,
    ///  The user's role (one of "anon", "user", "bot", "halfmod", "mod", "admin", "sysop").
    pub role: UserRole,
    /// The user's XP in points.
    pub xp: f64,
    /// The user's ISO 3166-1 country code, or null if hidden/unknown. Some vanity flags exist.
    pub country: Option<Arc<str>>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>,
    /// Whether this user is a verified account.
    pub verified: bool,
    /// This user's current TETRA LEAGUE standing:
    pub league: LeagueFullData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeagueFullPacketData {
    ///  The matched users:
    pub users: Box<[LeagueFullUser]>,
}

/// A packet containing a list of users and their TETRA LEAGUE standings.
pub type LeagueFullPacket = Packet<LeagueFullPacketData>;
