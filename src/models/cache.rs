use std::time::{UNIX_EPOCH, SystemTime, Duration};


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A cache object sent by the tetr.io api,
/// only the cached_until field is actually used by the cached http client.
pub struct Cache {
    pub status: String,
    pub cached_at: u128,
    pub cached_until: u128,
}

impl Cache {
    pub fn cached_for(duration: Duration) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("That can't be happening").as_millis();

        Cache {
            cached_at: now,
            status: String::from("cached"),
            cached_until: now + duration.as_millis()
        }
    }
}

impl Cache {
    pub fn time_until_elapsed(&self) -> Duration {
        Duration::from_millis((self.cached_until - SystemTime::now().duration_since(UNIX_EPOCH).expect("That can't be happening").as_millis()) as u64)
    }
}
