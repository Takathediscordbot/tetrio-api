//! 
//! [User Records](https://tetr.io/about/api/#usersuserrecords) models 


/// The model used for the 40L records of a user.
pub mod sprint_record;
/// The model used for the Blitz records of a user.
pub mod blitz_record;

use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

/// The user's 40 LINES record:
pub use self::sprint_record::SprintRecord;
/// The user's BLITZ record:
pub use self::blitz_record::BlitzRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// The requested user's ranked records:
pub struct UserRecordsInner {
    /// The user's 40 LINES record:
    #[serde(rename = "40l")]
    pub sprint: SprintRecord,
    /// The user's BLITZ record:
    pub blitz: BlitzRecord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// The user's ZEN record:
pub struct UserRecordsZen {
    /// The user's level in ZEN mode.
    pub level: i64,
    /// The user's score in ZEN mode.
    pub score: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// An object describing the user's single player records.
pub struct UserRecords {
    /// The requested user's ranked records:
    pub records: UserRecordsInner,
    /// The user's ZEN record:
    pub zen: UserRecordsZen,
}

/// The packet returned by the API when requesting a user's records.
pub type UserRecordsPacket = Packet<UserRecords>;
