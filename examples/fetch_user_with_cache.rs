
use tetrio_api::{http::clients::reqwest_client::InMemoryReqwestClient, models::packet::Packet};

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let client = InMemoryReqwestClient::default();
    
    // This returns an Arc because of cache
    let user = client.fetch_user_info("taka").await.expect("Couldn't fetch the CH.TETR.IO API to find the error! This could have been an error while parsing the data or while trying to send the HTTP request.");

    match &user {
        Packet { data: Some(_data), .. } => {

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
