
use std::{time::Duration, sync::Arc};

use super::cache::Cache;
use moka::Expiry;
use serde::{self, Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
/// A struct representing any packet sent by the tetr.io API as shown in the API docs.
/// I use it as a wrapper to all other models because the structure is always the same.
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

/// A struct used to automatically remove the cache entries when their expiration date has passed.
/// It is based around the cached_until field sent by the TETR.IO api, ensuring that the cache is always respected.
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
