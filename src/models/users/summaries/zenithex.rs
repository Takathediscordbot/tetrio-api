use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::ZenithExRecord};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenithExCareerBest {
    pub record: Option<ZenithExRecord>,
    pub rank: APIint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenithExSummary {
    pub record: Option<ZenithExRecord>,
    pub rank: APIint,
    pub rank_local: APIint,
    pub best: ZenithExCareerBest
}
