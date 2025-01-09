use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::SprintRecord};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintSummary {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<SprintRecord>,
    pub rank: APIint,
    pub rank_local: APIint
}
