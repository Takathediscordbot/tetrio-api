#![cfg(feature = "in_memory_cache")]
use std::{convert::Infallible, fmt::Debug, sync::Arc};

use async_trait::async_trait;
use moka::{future::Cache, Expiry};
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::error::Error, models::packet::{Packet, SuccessPacket}};

use super::cache::CacheHandler;



/// A struct used to automatically remove the cache entries when their expiration date has passed.
/// It is based around the cached_until field sent by the TETR.IO api, ensuring that the cache is always respected.
pub struct CacheExpiration;

impl<K, T> Expiry<K, Arc<SuccessPacket<T>>> for CacheExpiration {
    fn expire_after_create(&self, _key: &K, value: &Arc<SuccessPacket<T>>, _current_time: std::time::Instant) -> Option<std::time::Duration> {
         Some(value.cache.time_until_elapsed())
    }
}

pub struct MokaCache {
    cache: Cache<String, Arc<SuccessPacket<serde_json::Value>>>,
}

impl Default for MokaCache {
    fn default() -> Self {
        MokaCache {  
            cache: Cache::builder().expire_after(CacheExpiration).build()
        }
    }
}

#[async_trait]
impl<ErrorT: std::error::Error + Sync + Send + Debug> CacheHandler<ErrorT> for MokaCache {
    type CachingError = Infallible;
    async fn try_get_cache<T: DeserializeOwned + Serialize>(&self, cache_key: &str) -> Result<Option<Packet<T>>, Error<ErrorT, Self::CachingError>>
    {
        // Get cached value

        if let Some(json_value) = self.cache.get(cache_key).await {
            // Parse cache value
            let result = serde_json::from_value::<T>(json_value.data.clone()).map_err(Error::CacheConversionError);
                match result {
                    Ok(r) => {
                        // Return cached value
                        return Ok(Some(Packet {
                            success: json_value.success,
                            cache: Some(json_value.cache.clone()),
                            data: Some(r),
                            error: None
                        }));
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
        }

        Ok(None)
    }

    async fn cache_value<T: DeserializeOwned + Serialize + Send + Sync>(&self, cache_key: &str, cache_value: SuccessPacket<T>) -> Result<(), Error<ErrorT, Self::CachingError>> {
        let SuccessPacket { success, cache, data } = cache_value;
        self.cache.insert(cache_key.to_string(), Arc::new(SuccessPacket {
            success,
            cache,
            data: serde_json::to_value(data).map_err(|err| Error::ConversionError { error: err })?
        })).await;

        Ok(())
    }

}
