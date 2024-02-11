pub mod sprint_record;
pub mod blitz_record;
use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

pub use self::sprint_record::SprintRecord;
pub use self::blitz_record::BlitzRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserRecordsInner {
    #[serde(rename = "40l")]
    pub sprint: SprintRecord,
    pub blitz: BlitzRecord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserRecordsZen {
    pub level: i64,
    pub score: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserRecords {
    pub records: UserRecordsInner,
    pub zen: UserRecordsZen,
}

pub type UserRecordsPacket = Packet<UserRecords>;
