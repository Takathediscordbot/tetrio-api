#![allow(dead_code)]

pub mod http;
mod models;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    

    use crate::{http::value_bound_query::ValueBoundQuery};

    use super::http::client::{self as client};
    

    #[tokio::test]
    async fn fetch_user_info() {
        let fetched = client::fetch_user_info("takathedinosaur").await;
        let Ok(data) = fetched else {
            panic!("An error occured while trying to request user");
        };

        assert!(data.is_success() && data.into_success().is_some() && data.into_error().is_none());
    }

    #[tokio::test]
    async fn fail_fetch_user_info() {
        let fetched = client::fetch_user_info("KJBDEZHDIUZEHDIH").await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if user.into_success().is_some() {
                panic!("user was valid, test is worthless")
            } else {
            }
        }
    }

    #[tokio::test]
    async fn fetch_user_records() {
        let fetched = client::fetch_user_records("takathedinosaur").await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("user was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fail_fetch_user_records() {
        let fetched = client::fetch_user_records("KJBDEZHDIUZEHDIH").await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
                panic!("user was valid, test was useless")
            }
            else {
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_no_query() {
        let fetched = client::fetch_league_leaderboard(ValueBoundQuery::None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_country_query() {
        let fetched = client::fetch_league_leaderboard(
            ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_after_boundary() {
        let fetched = client::fetch_league_leaderboard(ValueBoundQuery::After { 
            after: ordered_float::OrderedFloat(22000.5), limit: None, country: None 
        }).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_before_boundary() {
        let fetched = client::fetch_league_leaderboard(
            ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_no_query() {
        let fetched = client::fetch_xp_leaderboard(ValueBoundQuery::None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_country_query() {
        let fetched = client::fetch_xp_leaderboard(
            ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_after_boundary() {
        let fetched = client::fetch_xp_leaderboard(ValueBoundQuery::After { 
            after: ordered_float::OrderedFloat(22000.5), limit: None, country: None 
        }).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_before_boundary() {
        let fetched = client::fetch_xp_leaderboard(
            ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    #[ignore = "Test takes too long"]
    async fn fetch_full_league_leaderboard() {
        let fetched = client::fetch_full_league_leaderboard(None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }   

    #[tokio::test]
    async fn fetch_full_league_leaderboard_country() {
        let fetched = client::fetch_full_league_leaderboard(Some("fr".to_string())).await;
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }   

}

#[cfg(test)] 
mod cached_tests {
    use crate::{http::value_bound_query::ValueBoundQuery};

    use crate::http::cached_client::CachedClient;    


    #[tokio::test]
    async fn fetch_user_info() {

        let client = CachedClient::default();
        let fetched = client.fetch_user_info("takathedinosaur").await;
        let Ok(data) = fetched else {
            panic!("An error occured while trying to request user");
        };

        assert!(data.is_success() && data.into_success().is_some() && data.into_error().is_none());

        let fetched = client.fetch_user_info("takathedinosaur").await;
        let Ok(data) = fetched else {
            panic!("An error occured while trying to request user");
        };

        assert!(data.is_success() && data.into_success().is_some() && data.into_error().is_none());
    }

    #[tokio::test]
    async fn fail_fetch_user_info() {
        let client = CachedClient::default();
        let fetched = client.fetch_user_info("KJBDEZHDIUZEHDIH").await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if user.into_success().is_some() {
                panic!("user was valid, test is worthless")
            } else {
            }
        }
    }

    #[tokio::test]
    async fn fetch_user_records() {
        let client = CachedClient::default();
        let fetched = client.fetch_user_records("takathedinosaur").await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("user was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fail_fetch_user_records() {
        let client = CachedClient::default();
        let fetched = client.fetch_user_records("KJBDEZHDIUZEHDIH").await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
                panic!("user was valid, test was useless")
            }
            else {
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_no_query() {
        let client = CachedClient::default();
        let fetched = client.fetch_league_leaderboard(ValueBoundQuery::None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_country_query() {
        let client = CachedClient::default();
        let fetched = client.fetch_league_leaderboard(
            ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_after_boundary() {
        let client = CachedClient::default();
        let fetched = client.fetch_league_leaderboard(ValueBoundQuery::After { 
            after: ordered_float::OrderedFloat(22000.5), limit: None, country: None 
        }).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_leaderboard_before_boundary() {
        let client = CachedClient::default();
        let fetched = client.fetch_league_leaderboard(
            ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None }
        ).await;
        assert!(&fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_no_query() {
        let client = CachedClient::default();
        let fetched = client.fetch_xp_leaderboard(ValueBoundQuery::None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_country_query() {
        let client = CachedClient::default();
        let fetched = client.fetch_xp_leaderboard(
            ValueBoundQuery::NotBound { limit: None, country: Some("fr".to_string()) }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_after_boundary() {
        let client = CachedClient::default();
        let fetched = client.fetch_xp_leaderboard(ValueBoundQuery::After { 
            after: ordered_float::OrderedFloat(24000.5), limit: None, country: None 
        }).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    async fn fetch_xp_leaderboard_before_boundary() {
        let client = CachedClient::default();
        let fetched = client.fetch_xp_leaderboard(
            ValueBoundQuery::Before { before: ordered_float::OrderedFloat(22000.5), limit: None, country: None }
        ).await;
        assert!(fetched.is_ok());
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }

    #[tokio::test]
    #[ignore = "Test takes too long"]
    async fn fetch_full_league_leaderboard() {
        let client = CachedClient::default();
        let fetched = client.fetch_full_league_leaderboard(None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }

        eprintln!("first full leaderboard!");

        let fetched = client.fetch_full_league_leaderboard(None).await;

        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }

        eprintln!("second full leaderboard!");

    }   

    #[tokio::test]
    async fn fetch_full_league_leaderboard_country() {
        let client = CachedClient::default();
        let fetched = client.fetch_full_league_leaderboard(Some("fr".to_string())).await;
        if let Ok(user) = fetched {
            if let Some(_user) = user.into_success() {
            }
            else {
                panic!("leaderboard was invalid")
            }
        }
    }   
}