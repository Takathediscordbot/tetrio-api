use std::fmt::Debug;

use bytes::Bytes;
use http::Request;

#[async_trait::async_trait]
pub trait HttpClient: 'static + Send + Sync {
    type HttpError: Debug + std::error::Error + Sync + Send;
    async fn execute(&self, request: Request<Vec<u8>>) -> Result<Bytes, Self::HttpError>;
}

