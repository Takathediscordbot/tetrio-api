use serde::{Deserialize, Serialize};

use crate::models::common::{APIfloat, APIint};

use std::collections::HashMap;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZenSummary {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub level: APIint,
    pub score: APIfloat
}
