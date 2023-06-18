use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDistinguishment {
    #[serde(rename = "type")]
    pub distinguishment_type: Arc<str>,
    pub detail: Option<Arc<str>>,
    pub header: Option<Arc<str>>,
    pub footer: Option<Arc<str>>
}
