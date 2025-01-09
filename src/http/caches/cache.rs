use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::error::Error, models::packet::{Packet, SuccessPacket}};



#[async_trait]
pub trait CacheHandler<ErrorT: std::error::Error + Sync + Send + Debug> {
    type CachingError: std::error::Error + Sync + Send + Debug;
    /// A method to implement to get a value from the cache
    /// It should NOT return an error if the value is not in the cache
    /// If the value is not in the cache, it should return Ok(None)
    async fn try_get_cache<T: DeserializeOwned + Serialize>(&self, cache_key: &str) -> Result<Option<Packet<T>>, Error<ErrorT, Self::CachingError>>;

    /// A method to implement to set a value in the cache
    /// It should return an error if the value could not be set
    async fn cache_value<T: DeserializeOwned + Serialize + Send + Sync>(&self, cache_key: &str, cache_value: SuccessPacket<T>) -> Result<(), Error<ErrorT, Self::CachingError>>;
}
