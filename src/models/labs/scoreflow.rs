use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, packet::Packet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreFlowPoint(pub APIint, pub APIint, pub APIint);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreFlow {
    #[serde(rename = "startTime")]
    pub start_time: APIint,
    pub points: Vec<ScoreFlowPoint>,
}

pub type ScoreFlowPacket = Packet<ScoreFlow>;