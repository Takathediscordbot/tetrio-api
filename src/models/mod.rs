/// A module containing cache models fetched from the ch.tetr.io API
/// Almost all requests return a cache object, which is used to determine if the data is still valid.
pub mod cache;

/// A module containing models fetched from the ch.tetr.io API
/// All modules follow a similar structure defined in this module. 
pub mod packet;

/// A module containing user models fetched from the ch.tetr.io API
/// Those models are used for single user info or for leaderboard requests.
pub mod users;

/// A module containing stream models fetched from the ch.tetr.io API
/// Those models are used for replays, tetra league recent games etc.
/// Those models are not documented on the official API documentation and therefore are not documented here.
pub mod streams;

/// A module containing stream news fetched from the ch.tetr.io API
/// Those models are used for new world records, new player joining etc.
/// Those models are not documented on the official API documentation and therefore are not documented here.
pub mod news;

/// A module containing general models fetched from the ch.tetr.io API
/// Those models are used for general stats, general activity etc.
pub mod general;
