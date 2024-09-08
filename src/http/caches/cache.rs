use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::error::Error, models::packet::{Packet, SuccessPacket}};



#[async_trait]
pub trait CacheHandler<ErrorT: std::error::Error + Sync + Send + Debug> {
    type CachingError: std::error::Error + Sync + Send + Debug;
    async fn try_get_cache<T: DeserializeOwned + Serialize>(&self, cache_key: &str) -> Result<Option<Packet<T>>, Error<ErrorT, Self::CachingError>>;
    async fn cache_value<T: DeserializeOwned + Serialize + Send + Sync>(&self, cache_key: &str, cache_value: SuccessPacket<T>) -> Result<(), Error<ErrorT, Self::CachingError>>;
}
