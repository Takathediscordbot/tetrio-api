/*

*/


use serde::{Deserialize, Serialize};


use crate::{http::parameters::{personal_user_records::GameMode, value_bound_query::Prisecter}, models::common::{APIfloat, APIint, APIintarray, APIsmallint, APIstring}};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzAggregateStats {
    pub apm: APIfloat,
    pub pps: APIfloat,
    pub vsscore: APIfloat
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzTime {
    pub start: APIint,
    pub zero: bool,
    pub locked: bool,
    pub prev: APIint,
    pub frameoffset: Option<APIint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzClears {
    pub singles: APIsmallint,
    pub doubles: APIsmallint,
    pub triples: APIsmallint,
    pub quads: APIsmallint,
    pub pentas: Option<APIsmallint>,
    pub realtspins: APIsmallint,
    pub minitspins: APIsmallint,
    pub minitspinsingles: APIsmallint,
    pub tspinsingles: APIsmallint,
    pub minitspindoubles: APIsmallint,
    pub tspindoubles: APIsmallint,
    pub minitspintriples: Option<APIsmallint>,
    pub tspintriples: APIsmallint,
    pub minitspinquads: Option<APIsmallint>,
    pub tspinquads: APIsmallint,
    pub tspinpentas: Option<APIsmallint>,
    pub allclear: APIsmallint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzGarbage {
    pub sent: APIsmallint,
    pub sent_nomult: Option<APIsmallint>,
    pub maxspike: Option<APIsmallint>,
    pub maxspike_nomult: Option<APIsmallint>,
    pub received: APIsmallint,
    pub attack: Option<APIsmallint>,
    pub cleared: Option<APIsmallint>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzFinesse {
    pub combo: APIsmallint,
    pub faults:  APIsmallint,
    pub perfectpieces:  APIsmallint
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzZenith {
    pub altitude: APIint,
    pub rank: APIint,
    pub peakrank: APIint,
    pub avgrankpts: APIint,
    pub floor: APIint,
    pub targetingfactor: APIint,
    pub targetinggrace: APIint,
    pub totalbonus: APIint,
    pub revives: APIint,
    #[serde(rename="revivesTotal")]
    pub revives_total: APIint,
    pub speedrun: bool,
    pub speedrun_seen: bool,
    pub splits: APIintarray
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzStats {
    pub seed: Option<APIfloat>,
    pub lines: APIint,
    pub level_lines: APIint,
    pub level_lines_needed: APIint,
    pub inputs: APIint,
    pub holds: Option<APIint>,
    pub time: Option<BlitzTime>,
    pub score: APIint,
    pub zenlevel: Option<APIint>,
    pub zenprogress: Option<APIint>,
    pub level: APIint,
    pub combo: APIint,
    pub currentcombopower: Option<APIint>,
    pub topcombo: APIint,
    pub btb: APIint,
    pub btbpower: Option<APIint>,
    pub combopower: Option<APIint>,
    pub topbtb: APIint,
    pub currentbtbchainpower: Option<APIint>,
    pub tspins: APIint,
    pub piecesplaced: APIint,
    pub clears: BlitzClears,
    pub garbage: BlitzGarbage,
    pub kill: Option<APIint>,
    pub kills: Option<APIint>,
    pub finesse: Option<BlitzFinesse>,
    pub finaltime: APIfloat,
    pub zenith: Option<BlitzZenith>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzResults {
    pub aggregatestats: BlitzAggregateStats,
    pub stats: BlitzStats,
    pub gameoverreason: APIstring

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecordUser {
    pub id: APIstring,
    pub username: APIstring,
    pub avatar_revision: Option<APIint>,
    pub banner_revision: Option<APIint>,
    pub country: Option<APIstring>,
    pub supporter: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzExtras {}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlitzRecord {
    #[serde(rename = "_id")]
    pub id: APIstring,
    pub replayid: APIstring,
    pub stub: bool,
    pub gamemode: GameMode,
    pub pb: bool,
    pub oncepb: bool,
    pub ts: APIstring,
    pub revolution: Option<APIstring>,
    pub user: Option<BlitzRecordUser>,
    pub otherusers: Vec<BlitzRecordUser>,
    pub leaderboards: Vec<APIstring>,
    pub results: BlitzResults,
    pub extras: BlitzExtras,
    pub disputed: bool,
    pub p: Prisecter
}
