use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::BlitzRecord};



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzSummary {
    pub record: Option<BlitzRecord>,
    pub rank: APIint,
    pub rank_local: APIint
}
