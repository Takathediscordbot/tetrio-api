use achievements::AchievementsSummary;
use blitz::BlitzSummary;
use serde::{Deserialize, Serialize};
use sprint::SprintSummary;
use tetra_league::LeagueSummary;
use zen::ZenSummary;
use zenith::ZenithSummary;
use zenithex::ZenithExSummary;

use crate::models::packet::Packet;


pub mod sprint;
pub mod blitz;
pub mod zenith;
pub mod zenithex;
pub mod tetra_league;
pub mod zen;
pub mod achievements;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllSummaries {
    /* See User Summary: 40 LINES. */
    #[serde(rename = "40l")]
    pub sprint: SprintSummary,
    /* See User Summary: BLITZ. */
    pub blitz: BlitzSummary,
    /* See User Summary: QUICK PLAY. */
    pub zenith: ZenithSummary,
    /* See User Summary: EXPERT QUICK PLAY. */
    pub zenithex: ZenithExSummary,
    /* See User Summary: TETRA LEAGUE. */
    pub league: LeagueSummary,
    /* See User Summary: ZEN. */
    pub zen: ZenSummary,
    /* See User Summary: Achievements. */
    pub achievements: AchievementsSummary,
}

pub type AllSummariesPacket = Packet<AllSummaries>;
pub type SprintSummaryPacket = Packet<SprintSummary>;
pub type BlitzSummaryPacket = Packet<BlitzSummary>;
pub type ZenithSummaryPacket = Packet<ZenithSummary>;
pub type ZenithExSummaryPacket = Packet<ZenithExSummary>;
pub type LeagueSummaryPacket = Packet<LeagueSummary>;
pub type ZenSummaryPacket = Packet<ZenSummary>;
pub type AchievementsSummaryPacket = Packet<AchievementsSummary>;
