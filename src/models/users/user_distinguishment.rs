
use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDistinguishment {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    #[serde(rename = "type")]
    pub distinguishment_type: APIstring,
    pub detail: Option<APIstring>,
    pub header: Option<APIstring>,
    pub footer: Option<APIstring>,
    
}
