/// [General Stats](https://tetr.io/about/api/#generalstats) model

use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    /// The amount of users on the server, including anonymous accounts.
    pub usercount: i64,
    /// The amount of users created a second (through the last minute).
    pub usercount_delta: f64,
    /// The amount of anonymous accounts on the server.
    pub anoncount: i64,
    /// The amount of ranked (visible in TETRA LEAGUE leaderboard) accounts on the server.
    pub rankedcount: i64,
    /// The amount of replays stored on the server.
    pub replaycount: i64,
    /// The amount of games played across all users, including both off- and online modes.
    pub gamesplayed: i64,
    /// The amount of games played a second (through the last minute).
    pub gamesplayed_delta: f64,
    /// The amount of games played across all users, including both off- and online modes, excluding games that were not completed (e.g. retries)
    pub gamesfinished: i64,
    /// The amount of seconds spent playing across all users, including both off- and online modes.
    pub gametime: f64,
    /// The amount of keys pressed across all users, including both off- and online modes.
    pub inputs: i64,
    pub piecesplaced: i64,
    ///  The total amount of accounts ever created (including pruned anons etc.).
    pub totalaccounts: i64,
}

/// The packet returned by the API when requesting the server stats.
pub type StatsPacket = Packet<Stats>;
