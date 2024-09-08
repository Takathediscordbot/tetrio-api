use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prisecter {
    pub pri: f64,
    pub sec: f64,
    pub ter: f64,
}

impl ToString for Prisecter {
    fn to_string(&self) -> String {
        return format!("{}:{}:{}", self.pri, self.sec, self.ter)
    }
}


pub enum ValueBoundQuery {
    After {
        after: Prisecter,
        limit: Option<i64>,
        country: Option<String>,
    },
    Before {
        before: Prisecter,
        limit: Option<i64>,
        country: Option<String>,
    },
    NotBound {
        limit: Option<i64>,
        country: Option<String>,
    },
    None,
}

impl ValueBoundQuery {
    pub fn as_query_params(self) -> Vec<[String; 2]> {
        let mut result = vec![];
        match self {
            ValueBoundQuery::After {
                after,
                limit,
                country,
                ..
            } => {
                result.push(["after".to_string(), after.to_string()]);
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };

                if let Some(country) = country {
                    result.push(["country".to_string(), country.to_string()]);
                };
            }
            ValueBoundQuery::Before {
                before,
                limit,
                country,
                ..
            } => {
                result.push(["before".to_string(), before.to_string()]);
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };

                if let Some(country) = country {
                    result.push(["country".to_string(), country.to_string()]);
                };
            }
            ValueBoundQuery::NotBound { limit, country } => {
                if let Some(limit) = limit {
                    result.push(["limit".to_string(), limit.to_string()]);
                };

                if let Some(country) = country {
                    result.push(["country".to_string(), country.to_string()]);
                };
            }
            ValueBoundQuery::None => {},
        };

        return result;
    }
}
