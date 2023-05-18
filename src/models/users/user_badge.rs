use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBadge {
    pub id: String, 
    pub label: String,
    pub ts: Option<serde_json::Value>
}