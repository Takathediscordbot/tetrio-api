use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{common::{APIfloat, APIint, APIstring}, users::user_rank::UserRank};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LeagueSummaryPast {
    /* The season ID. */
    pub season: APIstring,
    /* The username the user had at the time. */
    pub username: APIstring,
    /* The country the user represented at the time. */
    pub country: Option<APIstring>,
    /* This user's final position in the season's global leaderboards. */
    pub placement: Option<APIint>,
    /* Whether the user was ranked at the time of the season's end. */
    pub ranked: bool,
    /* The amount of TETRA LEAGUE games played by this user. */
    pub gamesplayed: Option<APIint>,
    /* The amount of TETRA LEAGUE games won by this user. */
    pub gameswon: Option<APIint>,
    /* This user's final Glicko-2 rating. */
    pub glicko: Option<APIfloat>,
    /* This user's final Glicko-2 Rating Deviation. */
    pub rd: Option<APIfloat>,
    /* This user's final TR (Tetra Rating). */
    pub tr: Option<APIfloat>,
    /* This user's final GLIXARE score (a % chance of beating an average player). */
    pub gxe: Option<APIfloat>,
    /* This user's final letter rank. z is unranked. */
    pub rank: APIstring,
    /* This user's highest achieved rank in the season. */
    pub bestrank: Option<APIstring>,
    /* This user's average APM (attack per minute) over the last 10 games in the season. */
    pub apm: APIfloat,
    /* This user's average PPS (pieces per second) over the last 10 games in the season. */
    pub pps: APIfloat,
    /* This user's average VS (versus score) over the last 10 games in the season. */
    pub vs: APIfloat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LeagueSummary {
    /* The amount of TETRA LEAGUE games played by this user. */
    pub gamesplayed: Option<APIint>,
    /* The amount of TETRA LEAGUE games won by this user. */
    pub gameswon: Option<APIint>,
    /* This user's Glicko-2 rating, or -1 if less than 10 games were played. */
    pub glicko: Option<APIfloat>,
    /* This user's Glicko-2 Rating Deviation, or -1 if less than 10 games were played. If over 100, this user is unranked. */
    pub rd: Option<APIfloat>,
    /* Whether this user's RD is rising (has not played in the last week). */
    pub decaying: Option<bool>,
    /* This user's TR (Tetra Rating), or -1 if less than 10 games were played. */
    pub tr: Option<APIfloat>,
    /* This user's GLIXARE score (a % chance of beating an average player), or -1 if less than 10 games were played. */
    pub gxe: Option<APIfloat>,
    /* This user's letter rank. z is unranked. */
    pub rank: Option<UserRank>,
    /* This user's highest achieved rank this season. */
    pub bestrank: Option<UserRank>,
    /* This user's average APM (attack per minute) over the last 10 games. */
    pub apm: Option<APIfloat>,
    /* This user's average PPS (pieces per second) over the last 10 games. */
    pub pps: Option<APIfloat>,
    /* This user's average VS (versus score) over the last 10 games. */
    pub vs: Option<APIfloat>,
    /* This user's position in global leaderboards, or -1 if not applicable. */
    pub standing: Option<APIint>,
    /* This user's position in local leaderboards, or -1 if not applicable. */
    pub standing_local: Option<APIint>,
    /* This user's percentile position (0 is best, 1 is worst). */
    pub percentile: Option<APIfloat>,
    /* This user's percentile rank, or z if not applicable. */
    pub percentile_rank: Option<UserRank>,
    /* The next rank this user can achieve, if they win more games, or null if unranked (or the best rank). */
    pub next_rank: Option<UserRank>,
    /* The previous rank this user can achieve, if they lose more games, or null if unranked (or the worst rank). */
    pub prev_rank: Option<UserRank>,
    /* The position of the best player in the user's current rank, surpass them to go up a rank. -1 if unranked (or the best rank). */
    pub next_at: Option<APIint>,
    /* The position of the worst player in the user's current rank, dip below them to go down a rank. -1 if unranked (or the worst rank). */
    pub prev_at: Option<APIint>,
    pub past: Option<HashMap<APIstring, LeagueSummaryPast>>
}
