use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDistinguishment {
    #[serde(rename = "type")]
    pub identifier: String,
}
