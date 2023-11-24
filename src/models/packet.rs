
use std::{time::Duration, sync::Arc};

use super::cache::Cache;
use moka::Expiry;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuccessPacket<T> {
    pub success: bool,
    pub cache: Cache,
    pub data: T,
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorPacket {
    pub success: bool,
    pub error: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Packet<T> {
    pub success: bool,
    pub cache: Option<Cache>,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> Packet<T> {

    pub fn is_success(&self) -> bool {
        self.success
    }
}

pub struct CacheExpiration;

impl<K, T> Expiry<K, Arc<Packet<T>>> for CacheExpiration {
    fn expire_after_create(&self, _key: &K, value: &Arc<Packet<T>>, _current_time: std::time::Instant) -> Option<std::time::Duration> {
         match &value.cache {
            Some(a) => {
                 Some(a.time_until_elapsed())
                
            },
            None => Some(Duration::from_secs(0)),
        }               
    }
}
