use std::sync::Arc;

use crate::models::packet::Packet;
use crate::models::users::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XpUser {
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    pub username: Arc<str>,
    pub role: UserRole,
    pub ts: Option<Arc<str>>,
    pub country: Option<Arc<str>>,
    pub supporter: Option<bool>,
    pub verified: bool,
    pub xp: f64,
    pub gamesplayed: i64,
    pub gameswon: i64,
    pub gametime: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XpPacketData {
    pub users: Box<[XpUser]>,
}

pub type XpPacket = Packet<XpPacketData>;
