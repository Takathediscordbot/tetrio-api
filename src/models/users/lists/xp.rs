//!
//! [XP Leaderboard](https://tetr.io/about/api/#userlistsxp) models

use std::sync::Arc;

use crate::models::packet::Packet;
use crate::models::users::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
/// The matched user's data
pub struct XpUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    /// The user's username.
    pub username: Arc<str>,
    /// The user's role (one of "anon", "user", "bot", "halfmod", "mod", "admin", "sysop").
    pub role: UserRole,
    /// When the user account was created. If not set, this account was created before join dates were recorded.
    pub ts: Option<Arc<str>>,
    /// The user's ISO 3166-1 country code, or null if hidden/unknown. Some vanity flags exist.
    pub country: Option<Arc<str>>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>,
    ///  Whether this user is a verified account.
    pub verified: bool,
    /// The user's XP in points.
    pub xp: f64,
    ///  The amount of online games played by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gamesplayed: i64,
    /// The amount of online games won by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gameswon: i64,
    /// The amount of seconds this user spent playing, both on- and offline. If the user has chosen to hide this statistic, it will be -1.
    pub gametime: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XpPacketData {
    /// The matched users:
    pub users: Box<[XpUser]>,
}

/// A packet containing XP leaderboard data.
pub type XpPacket = Packet<XpPacketData>;
