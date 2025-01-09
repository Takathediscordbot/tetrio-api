use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{common::APIint, users::user_records::ZenithRecord};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZenithCareerBest {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<ZenithRecord>,
    pub rank: APIint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZenithSummary {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<ZenithRecord>,
    pub rank: APIint,
    pub rank_local: APIint,
    pub best: ZenithCareerBest
}
