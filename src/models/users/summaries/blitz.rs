use serde::{Deserialize, Serialize};

use crate::models::{common::APIint, users::user_records::BlitzRecord};
use std::collections::HashMap;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlitzSummary {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<BlitzRecord>,
    pub rank: APIint,
    pub rank_local: APIint
}
