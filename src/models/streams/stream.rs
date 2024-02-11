//! [Stream](https://tetr.io/about/api/#streamsstream) model
//! 
//! As the documentation for those models is not available, the fields are not documented.
//! You will have to test the routes yourself to figure out which fields you can get.

use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    pub records: Vec<serde_json::Value>,
}

pub type StreamPacket = Packet<Stream>;
