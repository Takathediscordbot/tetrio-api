
use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBadge {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub id: APIstring,
    pub label: APIstring,
    pub ts: Option<serde_json::Value>,
    pub group: Option<APIstring>,
    pub desc: Option<APIstring>,
    pub global: Option<bool>,
}
