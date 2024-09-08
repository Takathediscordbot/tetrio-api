pub enum LeaderboardType {
    // TETRA League Leaderboard
    League,
    // XP leaderboard
    Xp,
    // Achivement rating leaderboard
    Ar
}

pub enum HistoricalLeaderboardType {
    // TETRA League Leaderboard
    League
}


pub struct RecordLeaderboardQuery  {

}

impl ToString for LeaderboardType {
    fn to_string(&self) -> String {
        match self {
            Self::League => "league".to_string(),
            Self::Xp => "xp".to_string(),
            Self::Ar => "ar".to_string()
        }
    }
}

impl ToString for HistoricalLeaderboardType {
    fn to_string(&self) -> String {
        match self {
            Self::League => "league".to_string(),
        }
    }
}

