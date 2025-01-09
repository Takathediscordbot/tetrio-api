use serde::{Deserialize, Serialize};

use crate::models::common::{APIfloat, APIint, APIstring};
use std::collections::HashMap;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Achievement {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    #[serde(rename = "_id")]
    pub id: Option<APIstring>,
    /* The Achievement ID, for every type of achievement. */
    pub k: APIint,
    /* The category of the achievement. */
    pub category: APIstring,
    /* The primary name of the achievement. */
    pub name: APIstring,
    /* The objective of the achievement. */
    pub object: APIstring,
    /* The flavor text of the achievement. */
    pub desc: APIstring,
    /* The order of this achievement in its category. */
    pub o: Option<APIint>,
    /*  The rank type of this achievement */
    pub rt: u8,
    /* The achieved score for this achievement (replaces V) */
    pub vt: u8,
    /* The AR type of this achievement */
    pub art: u8,
    /* The minimum score required to obtain the achievement. */
    pub min: APIint,
    /* The amount of decimal placed to show. */
    pub deci: APIint,
    /* Whether this achievement is usually not shown. */
    pub hidden: bool,

    pub v: Option<APIfloat>,
    /* Additional data (see score). */
    pub a: Option<APIfloat>,
    /* The time the achievement was updated. */
    pub t: Option<APIstring>,
    /* The zero-indexed position in the achievement's leaderboards. */
    pub pos: Option<APIint>,
    /* The total amount of players who have this achievement (with a value of min or higher). */
    pub total: Option<APIint>,
    /* The rank of the achievement */
    pub rank: Option<u8>,
    pub n: Option<APIstring>,
    pub nolb: bool,
    pub stub: Option<bool>,
    pub disabled: Option<bool>,
    pub event: Option<APIstring>,
    pub event_past: Option<bool>,

}

pub type AchievementsSummary = Vec<Achievement>;
