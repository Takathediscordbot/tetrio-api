use std::fmt::{Debug, Display};


#[derive(Debug)]
pub enum Error<HttpError: Debug + Send + Sync, CachingError: Debug + Send + Sync> {
    HttpError(HttpError),
    CachingError(CachingError),
    CacheConversionError(serde_json::Error),
    RequestParsingError(http::Error),
    InvalidHeaderValue(http::header::InvalidHeaderValue),
    ParsingError(serde_path_to_error::Error<serde_json::Error>),
    ConversionError(serde_path_to_error::Error<serde_json::Error>),
    SerdeError(serde_json::Error),
}



impl<HttpError: StdError + Debug + Send + Sync, CachingError: StdError + Debug + Send + Sync> Display for Error<HttpError, CachingError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::InvalidHeaderValue(error) => write!(f, "InvalidHeaderValue: {error}"),
            Error::RequestParsingError(error) => write!(f, "RequestParsingError: {}", error),
            Error::CacheConversionError(error) => write!(f, "Couldn't convert cache entry: {}", error),
            Error::ParsingError(error) => {
                write!(f, "ParsingError: Path: {}, error: {}", error.path(), error.inner())
            },
            Error::ConversionError(error)=> write!(f, "Couldn't convert cache entry: path: {}, error: {}", error.path(), error.inner()),
            Error::HttpError(error) => write!(f, "HttpError: {error}"),
            Error::CachingError(error) => write!(f, "CachingError: {error}"),
            Error::SerdeError(error) => write!(f, "SerdeError: {error}"),
        }
    }
}

use std::error::Error as StdError;


impl<HttpError: StdError + Debug + Send + Sync, CachingError: StdError + Debug + Send + Sync> StdError for Error<HttpError, CachingError> {}

pub trait ErrorTrait {
    type Error;
}