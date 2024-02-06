/// The module which actually lets you interact with the TETR.IO api
/// You will find in here both a classic HTTP client which does not manage the cache 
/// and a HTTP client which stores the API responses in an in memory cache automatically
/// You should always prefer using the cached http client to make sure to not spam the API.
pub mod http;

/// The models used by this crate
/// You will be able to find any models returned by API functions here
/// There might be some innacuracies compared to the models in the [TETR.IO API Specs](https://tetr.io/about/api/)
pub mod models;


