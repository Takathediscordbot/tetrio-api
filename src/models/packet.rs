
use std::{time::Duration, sync::Arc};

use super::cache::Cache;
use moka::Expiry;
use serde::{self, Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// A struct representing any packet sent by the tetr.io API as shown in the API docs.
/// I use it as a wrapper to all other models because the structure is always the same.
pub struct Packet<T> {
    /// Whether the request was valid.
    pub success: bool,
    /// Cache data for this packet. Includes a cached_at and cached_untill allowing me to automatically cache the data.
    pub cache: Option<Cache>,
    /// The data sent by the tetrio api. Usually the data would only be set if success is true, 
    /// success can however be true meanwhile data is still none. (For example when searching a user with a discord id, the user might not have linked their discord account but the request would still set success to true.)
    pub data: Option<T>,
    /// An error sent by the tetrio api. As far as I know, there can only be an error if success is false.
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
