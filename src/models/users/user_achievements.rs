use serde::{Deserialize, Serialize};

use crate::models::common::APIint;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserArCounts {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    #[serde(rename="1")]
    bronze: Option<APIint>,
    #[serde(rename="2")]
    silver: Option<APIint>,
    #[serde(rename="3")]
    gold: Option<APIint>,
    #[serde(rename="4")]
    platinum: Option<APIint>,
    #[serde(rename="5")]
    diamond: Option<APIint>,
    #[serde(rename="100")]
    issued: Option<APIint>,
    
    t100: Option<APIint>,
    t50: Option<APIint>,
    t25: Option<APIint>,
    t10: Option<APIint>,
    t5: Option<APIint>,
    t3: Option<APIint>,
}
