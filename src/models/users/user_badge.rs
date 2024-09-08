
use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserBadge {
    pub id: APIstring,
    pub label: APIstring,
    pub ts: Option<serde_json::Value>,
    pub group: Option<APIstring>
}
