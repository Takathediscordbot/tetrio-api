use crate::models::{common::APIintarray, packet::Packet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Activity {
    pub activity: APIintarray,
}

pub type ActivityPacket = Packet<Activity>;
