use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDistinguishment {
    #[serde(rename = "type")]
    pub distinguishment_type: String,
    pub detail: Option<String>,
    pub header: Option<String>,
    pub footer: Option<String>
}
