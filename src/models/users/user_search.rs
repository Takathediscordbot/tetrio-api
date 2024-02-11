//! [User search](https://tetr.io/about/api/#userssearchquery) models 

use std::sync::Arc;

use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserSearch {
    ///  The user's internal ID.
    #[serde(rename = "_id")]
    pub id: Arc<str>,
    ///  The user's username.
    pub username: Arc<str>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserSearchPacketData {
    /// The user data
    pub user: UserSearch,
}

pub type UserSearchPacket = Packet<UserSearchPacketData>;
