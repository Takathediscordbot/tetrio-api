use serde::{Deserialize, Serialize};

use crate::models::common::{APIfloat, APIint};




#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenSummary {
    pub level: APIint,
    pub score: APIfloat
}
