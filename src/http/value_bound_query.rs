
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueBoundQuery {
    After {
        after: ordered_float::OrderedFloat<f64>,
        limit: Option<i64>,
        country: Option<String>,
    },
    Before {
        before: ordered_float::OrderedFloat<f64>,
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
    pub fn as_query_string(&self) -> String {
        match self {
            ValueBoundQuery::After {
                after,
                limit,
                country,
            } => {
                let mut result = format!("?after={after}");
                if let Some(limit) = limit {
                    result += format!("&limit={limit}").as_str();
                };

                if let Some(country) = country {
                    result += format!("&country={}", country.to_uppercase()).as_str();
                }

                result
            }
            ValueBoundQuery::Before {
                before,
                limit,
                country,
            } => {
                let mut result = format!("?before={before}");
                if let Some(limit) = limit {
                    result += format!("&limit={limit}").as_str();
                };

                if let Some(country) = country {
                    result += format!("&country={country}").as_str();
                }

                result
            }
            ValueBoundQuery::NotBound { limit, country } => {
                let mut result = String::new();
                if let Some(limit) = limit {
                    result += format!("?limit={limit}").as_str();
                    if let Some(country) = country {
                        result += format!("&country={country}").as_str();
                    }
                } else if let Some(country) = country {
                    result += format!("?country={country}").as_str();
                }

                result
            }
            ValueBoundQuery::None => String::new(),
        }
    }
}
