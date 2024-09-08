use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stream {
    pub records: Vec<serde_json::Value>,
}

pub type StreamPacket = Packet<Stream>;
