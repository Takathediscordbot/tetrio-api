
use std::time::Duration;

pub mod http;
pub mod models;


/// This is only used to make tests not get rate limited. It needs to be public to be accessed by doctest
pub fn delay_test() {
    std::thread::sleep(Duration::from_millis(500));
}