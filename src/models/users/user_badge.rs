use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBadge {
    pub id: Box<str>, 
    pub label: Box<str>,
    pub ts: Option<serde_json::Value>
}