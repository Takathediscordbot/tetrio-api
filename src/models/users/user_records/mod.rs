pub mod sprint_record;
pub mod blitz_record;
pub mod zenith_record;
pub mod zenithex_record;
pub mod league_record;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::common::APIArray;
use crate::models::packet::Packet;

pub use self::sprint_record::SprintRecord;
pub use self::blitz_record::BlitzRecord;
pub use self::zenith_record::ZenithRecord;
pub use self::zenithex_record::ZenithExRecord;
pub use self::league_record::LeagueRecord;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PersonalUserRecords<T> {

    #[serde(flatten)]
    pub ignored_fields: HashMap<String, serde_json::Value>,
    pub entries: APIArray<T>,
}

pub type PersonalSprintRecordPacket = Packet<PersonalUserRecords<SprintRecord>>;
pub type PersonalBlitzRecordPacket = Packet<PersonalUserRecords<BlitzRecord>>;
pub type PersonalZenithRecordPacket = Packet<PersonalUserRecords<ZenithRecord>>;
pub type PersonalZenithExRecordPacket = Packet<PersonalUserRecords<ZenithExRecord>>;
pub type PersonalLeagueRecordPacket = Packet<PersonalUserRecords<LeagueRecord>>;

