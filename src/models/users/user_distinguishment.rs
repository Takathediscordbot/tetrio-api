
use serde::{Deserialize, Serialize};

use crate::models::common::APIstring;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserDistinguishment {
    #[serde(rename = "type")]
    pub distinguishment_type: APIstring,
    pub detail: Option<APIstring>,
    pub header: Option<APIstring>,
    pub footer: Option<APIstring>
}
