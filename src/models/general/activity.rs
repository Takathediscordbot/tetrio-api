use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Activity {
    pub activity: Vec<i64>,
}

pub type ActivityPacket = Packet<Activity>;
