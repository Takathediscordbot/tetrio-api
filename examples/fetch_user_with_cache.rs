use std::f64::NAN;

use tetrio_api::{http::cached_client::CachedClient, models::packet::Packet};

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let client = CachedClient::default();
    
    // This returns an Arc because of cache
    let user = client.fetch_user_info("takathedinosaur").await.expect("Couldn't fetch the CH.TETR.IO API to find the error! This could have been an error while parsing the data or while trying to send the HTTP request.");

    match user.as_ref() {
        Packet { data: Some(data), .. } => {

            println!("{} (id: {}): {}pps, {}apm {}vs", data.user.username, data.user.id, data.user.league.pps.unwrap_or(NAN), data.user.league.apm.unwrap_or(NAN), data.user.league.vs.unwrap_or(NAN));
        },
        Packet { success, error, .. } => {
            if *success {
                eprintln!("The API didn't return an error but no data was found somehow!"); 
                // According to the API documentation that is not a possible case.
                unreachable!();
            }

            eprintln!("An error has occured while trying to fetch the user! {:?}", error)            
        }
    };


}