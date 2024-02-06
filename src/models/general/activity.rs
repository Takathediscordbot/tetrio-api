use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Activity {
    pub activity: Vec<i64>,
}

pub type ActivityPacket = Packet<Activity>;
