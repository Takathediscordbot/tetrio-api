use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    pub records: Vec<serde_json::Value>,
}

pub type StreamPacket = Packet<Stream>;
