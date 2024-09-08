use std::{borrow::Cow, fmt::Debug};

use async_trait::async_trait;
use redis::Commands;
use serde::{de::DeserializeOwned, Serialize};

use crate::{http::error::Error, models::packet::{Packet, SuccessPacket}};

use super::cache::CacheHandler;

pub struct RedisCache<'a> {
    pub client: Cow<'a, redis::Client>
}

impl<'a> RedisCache<'a> {
    pub fn new(client: Cow<'a, redis::Client>) -> Self {
        Self {
            client
        }
    }
}

#[async_trait]
impl<'a, ErrorT: std::error::Error + Sync + Send + Debug> CacheHandler<ErrorT> for RedisCache<'a> {
    type CachingError = redis::RedisError;
    async fn try_get_cache<T: DeserializeOwned + Serialize>(&self, key: &str) -> Result<Option<Packet<T>>, Error<ErrorT, Self::CachingError>>
    {
        let mut con = self.client.get_connection().map_err(Error::CachingError)?;

        let exists: bool = con.exists(key).map_err(Error::CachingError)?;
        
        match exists {
            false => Ok(None),
            true => Ok(Some(serde_json::from_str(&con.get::<&str, String>(key).map_err(Error::CachingError)?).map_err(Error::CacheConversionError)?))
        }
    }

    async fn cache_value<T: DeserializeOwned + Serialize + Send + Sync>(&self, key: &str, value: SuccessPacket<T>) -> Result<(), Error<ErrorT, Self::CachingError>> {
        let mut con = self.client.get_connection().map_err(Error::CachingError)?;
        
        con.set_ex::<_, _, redis::Value>(key, serde_json::to_string(&value).map_err(Error::CacheConversionError)?, value.cache.time_until_elapsed().as_secs()).map_err(Error::CachingError)?;

        Ok(())
    }

}