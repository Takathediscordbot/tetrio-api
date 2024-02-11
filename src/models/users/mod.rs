//! These modules all include individual part of what makes a TETR.IO user

/// This module represents the different list of users that can be requested
/// Includes tetra league leaderboards and xp leaderboards
pub mod lists;
/// This module represents the User Badges models that can be included in a user info payload.
pub mod user_badge;
/// This module represents the User Connections models that can be included in a user info payload.
/// For now it only includes a discord connection
pub mod user_connections;

/// This module represents the user distinguishments, usually represented by banners.
/// This is a part of the user info payload
pub mod user_distinguishment;

/// This module represents the user info payload, which is the main payload for a user request.
pub mod user_info;
/// This module represents the user tetra league ranks, it can be included in multiple types of payloads such as
/// user info and leaderboards.
pub mod user_rank;

/// This module represents the user records models that can be included in a userrecords payload
/// It includes the user's best records in different game modes such as 40L, Blitz and Zen.
pub mod user_records;

/// This module represents the different roles that a user can have, mostly as part of the tetrio team
/// It is used in the user info payload
pub mod user_role;

/// This module represents the user search payload, which is the main payload for a user search request.
/// It can be used to find a tetrio user with their connections to other platforms (for now only discord).
pub mod user_search;
