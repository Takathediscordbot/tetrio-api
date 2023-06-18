use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDistinguishment {
    #[serde(rename = "type")]
    pub distinguishment_type: Box<str>,
    pub detail: Option<Box<str>>,
    pub header: Option<Box<str>>,
    pub footer: Option<Box<str>>
}
