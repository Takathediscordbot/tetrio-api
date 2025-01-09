use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{common::APIint, users::user_records::ZenithExRecord};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZenithExCareerBest {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<ZenithExRecord>,
    pub rank: APIint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZenithExSummary {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub record: Option<ZenithExRecord>,
    pub rank: APIint,
    pub rank_local: APIint,
    pub best: ZenithExCareerBest
}
