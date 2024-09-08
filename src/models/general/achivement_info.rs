use serde::{Deserialize, Serialize};

use crate::models::{
    common::{APIArray, APIfloat, APIint, APIstring}, packet::Packet, users::summaries::achievements::Achievement
};

#[derive(Clone, Serialize, Deserialize)]
pub struct AchievementLeaderboardUser {
    /* The user's internal ID. */
    pub _id: APIstring,
    /* The user's username. */
    pub username: APIstring,
    /* The user's role. */
    pub role: APIstring,
    /* Whether the user is supporting TETR.IO. */
    pub supporter: bool,
    /* The user's country, if public. */
    pub country: Option<APIstring>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AchievementLeaderboard {
    /* The user owning the achievement: */
    pub u: AchievementLeaderboardUser,
    /* The achieved score in the achievement. */
    pub v: APIfloat,
    /* Additional score for the achievement. */
    pub a: Option<APIfloat>,
    /* The time the achievement was last updated. */
    pub t: APIstring,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AchievementCutoffs {
    /* The total amount of users with this achievement. */
    pub total: APIint,
    /* If applicable, the score required to obtain a Diamond rank. (If null, any score is allowed; if not given, this rank is not available.) */
    pub diamond: Option<APIfloat>,
    /* If applicable, the score required to obtain a Platinum rank. (If null, any score is allowed; if not given, this rank is not available.) */
    pub platinum: Option<APIfloat>,
    /* If applicable, the score required to obtain a Gold rank. (If null, any score is allowed; if not given, this rank is not available.) */
    pub gold: Option<APIfloat>,
    /* If applicable, the score required to obtain a Silver rank. (If null, any score is allowed; if not given, this rank is not available.) */
    pub silver: Option<APIfloat>,
    /* If applicable, the score required to obtain a Bronze rank. (If null, any score is allowed; if not given, this rank is not available.) */
    pub bronze: Option<APIfloat>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AchievementInfo {
    pub achievement: Achievement,
    pub leaderboard: APIArray<AchievementLeaderboard>,
    pub cutoffs: AchievementCutoffs,
}


pub type AchievementInfoPacket = Packet<AchievementInfo>;