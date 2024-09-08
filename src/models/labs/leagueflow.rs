use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, packet::Packet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueFlowPoint(pub APIint, pub APIint, pub APIint, pub APIint);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeagueFlow {
    #[serde(rename = "startTime")]
    pub start_time: APIint,
    pub points: Vec<LeagueFlowPoint>,
}

pub type LeagueFlowPacket = Packet<LeagueFlow>;
