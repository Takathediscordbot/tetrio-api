use crate::models::packet::Packet;
use crate::models::users::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XpUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub ts: Option<String>,
    pub country: Option<String>,
    pub supporter: Option<bool>,
    pub verified: bool,
    pub xp: f64,
    pub gamesplayed: i64,
    pub gameswon: i64,
    pub gametime: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XpPacketData {
    pub users: Vec<XpUser>,
}

pub type XpPacket = Packet<XpPacketData>;
