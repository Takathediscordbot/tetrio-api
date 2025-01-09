use std::{convert::Infallible, fmt::Debug};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::error::Error, models::packet::{Packet, SuccessPacket}};

use super::cache::CacheHandler;

/// A simple cache implementation that simply always does nothing.
pub struct NoopCache;

#[async_trait]
impl<ErrorT: std::error::Error + Sync + Send + Debug> CacheHandler<ErrorT> for NoopCache {
    type CachingError = Infallible;
    async fn try_get_cache<T: DeserializeOwned + Serialize>(&self, _: &str) -> Result<Option<Packet<T>>, Error<ErrorT, Self::CachingError>>
    {
        Ok(None)
    }


    async fn cache_value<T: DeserializeOwned + Serialize + Send + Sync>(&self, _: &str, _: SuccessPacket<T>) -> Result<(), Error<ErrorT, Self::CachingError>> {
        Ok(())
    }

}