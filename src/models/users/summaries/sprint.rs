use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::SprintRecord};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SprintSummary {
    pub record: Option<SprintRecord>,
    pub rank: APIint,
    pub rank_local: APIint
}
