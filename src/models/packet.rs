

use super::cache::Cache;
use serde::{self, de::DeserializeOwned, Deserialize, Serialize};

pub trait PacketData: DeserializeOwned + Serialize {}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    pub msg: String
}


#[derive(Debug, Clone, Deserialize, Serialize)]
/// A struct representing any packet sent by the tetr.io API as shown in the API docs.
/// I use it as a wrapper to all other models because the structure is always the same.
pub struct Packet<T>  {
    pub success: bool,
    pub cache: Option<Cache>,
    pub data: Option<T>,
    pub error: Option<Error>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A struct representing any packet sent by the tetr.io API as shown in the API docs.
/// I use it as a wrapper to all other models because the structure is always the same.
pub struct SuccessPacket<T> {
    pub success: bool,
    pub cache: Cache,
    pub data: T,
}




impl<T> Packet<T> {
    pub fn is_success(&self) -> bool {
        self.success
    }
}
