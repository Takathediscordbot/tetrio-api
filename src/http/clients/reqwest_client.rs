
#![cfg(feature = "reqwest_http_client")]

use bytes::Bytes;
use http::Request;

use async_trait::async_trait;
use reqwest::Request as Reqwest;

use crate::http::{cached_client::CachedClient, caches::{moka::MokaCache, noop_cache::NoopCache, redis_cache::RedisCache}, error::ErrorTrait};

use super::http_client::HttpClient;


/// A reqwest based http client
pub struct ReqwestClient {
    client: reqwest::Client
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self { client: Default::default() }
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    type HttpError = reqwest::Error;
    async fn execute(&self, request: Request<Vec<u8>>) -> Result<Bytes, Self::HttpError> {
        let req = Reqwest::try_from(request)?;
        self.client.execute(req).await?.bytes().await
    }

}

pub type UncachedReqwestClient = CachedClient<ReqwestClient, NoopCache>;
pub type UncachedReqwestError = <UncachedReqwestClient as ErrorTrait>::Error;

#[cfg(feature = "in_memory_cache")]
/// A reqwest based http client using an in-memory cache
pub type InMemoryReqwestClient = CachedClient<ReqwestClient, MokaCache>;
#[cfg(feature = "in_memory_cache")]
pub type InMemoryReqwestError = <InMemoryReqwestClient as ErrorTrait>::Error;

#[cfg(feature = "redis_cache")]
/// A reqwest based http client using a redis cache
pub type RedisReqwestClient<'a> = CachedClient<ReqwestClient, RedisCache<'a>>;
#[cfg(feature = "redis_cache")]
pub type RedisReqwestError<'a> = <RedisReqwestClient<'a> as ErrorTrait>::Error;