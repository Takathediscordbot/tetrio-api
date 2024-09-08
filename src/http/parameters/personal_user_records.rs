use serde::{Deserialize, Serialize};

use super::value_bound_query::Prisecter;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    #[serde(rename = "40l")]
    Sprint,
    Blitz,
    Zenith,
    ZenithEX,
    League,
}

impl ToString for GameMode {
    fn to_string(&self) -> String {
        match self {
            Self::Sprint => { return "40l".to_string(); },
            Self::Blitz => { return "blitz".to_string(); },
            Self::Zenith => { return "zenith".to_string(); },
            Self::ZenithEX => { return "zenithex".to_string(); },
            Self::League => { return "league ".to_string(); },
        };
    }
}

pub enum PersonalLeaderboard {
    Top,
    Recent,
    Progression
}

impl ToString for PersonalLeaderboard {
    fn to_string(&self) -> String {
        match self {
            Self::Top => { return "top".to_string(); },
            Self::Recent => { return "recent".to_string(); },
            Self::Progression => { return "progression".to_string(); },
        };
    }
}

pub enum PersonalRecordsQuery {
    After {
        after: Prisecter,
        limit: Option<i64>,
    },
    Before {
        before: Prisecter,
        limit: Option<i64>,
    },
    NotBound {
        limit: Option<i64>,
    },
    None,
}

impl PersonalRecordsQuery {
    pub fn as_query_params(self) -> Vec<[String; 2]> {
        let mut result = vec![];
        match self {
            PersonalRecordsQuery::After {
                after,
                limit,
                ..
            } => {
                result.push(["after".to_string(), after.to_string()]);
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };


            }
            PersonalRecordsQuery::Before {
                before,
                limit,
                ..
            } => {
                result.push(["before".to_string(), before.to_string()]);
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };

            }
            PersonalRecordsQuery::NotBound { limit,  } => {
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };


            }
            PersonalRecordsQuery::None => {},
        };

        return result;
    }
}
