use std::time::{UNIX_EPOCH, SystemTime};


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cache {
    pub status: String,
    pub cached_at: u64,
    pub cached_until: u64,
}

impl Cache {
    pub fn get_milliseconds_until_elapsed(&self) -> u64 {
        (self.cached_until - SystemTime::now().duration_since(UNIX_EPOCH).expect("That can't be happening").as_secs()) * 1000
    }
}
