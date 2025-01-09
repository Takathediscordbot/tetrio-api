/// Trait used to define your own cache handler
pub mod cache;
/// A caching method that does nothing. Useful for testing or if you don't want to cache anything.
pub mod noop_cache;

/// A redis caching implementation.
#[cfg(feature = "redis_cache")]
pub mod redis_cache;

/// An in-memory caching implementation.
#[cfg(feature = "in_memory_cache")]
pub mod moka;