use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBadge {
    pub id: Arc<str>, 
    pub label: Arc<str>,
    pub ts: Option<serde_json::Value>
}