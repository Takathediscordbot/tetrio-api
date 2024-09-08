use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::ZenithRecord};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenithCareerBest {
    pub record: Option<ZenithRecord>,
    pub rank: APIint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenithSummary {
    pub record: Option<ZenithRecord>,
    pub rank: APIint,
    pub rank_local: APIint,
    pub best: ZenithCareerBest
}
