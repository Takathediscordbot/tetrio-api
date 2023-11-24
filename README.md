# tetrio-api

[![codecov](https://codecov.io/gh/Takathediscordbot/tetrio-api/graph/badge.svg?token=NPGQ45PP4B)](https://codecov.io/gh/Takathediscordbot/tetrio-api)

A library to interact with the [TETR.IO public API](https://ch.tetr.io), with both a simple http client that doesn't manage cache and a "cached-http-client" that holds the data fetched from the API in memory. 
A library already exists to interact with the API in rust [here](https://github.com/Rinrin0413/tetr-ch-rs), it is easier to use because it has been published on crates.io and stuff, but I still needed an easier way to interact with the cache so I had to work on this library in the end.

The library tries to stay accurate to the [API specs](https://tetr.io/about/api/) but I had to take some liberties because: 
- I needed to access replay data, which are not documented
- Sometimes the documentation was partly innacurate which would cause errors when trying to use the library.

This library **might not** be perfect, but I did write tests, and since rust models are strict, the models have to comply with the current state of the API for the tests to pass, making it essentially accurate and usable.

## Quick start

### Install

You can add the package by cargo:

```bash
cargo add tetrio-api
```

Or add this to your Cargo.toml

```toml
[dependencies]
tetrio-api = { "git" = "https://github.com/Takathediscordbot/tetrio-api" }
```

This will automatically add the latest version of this api to your dependencies.

You can also use a specific commit like so:

```toml
[dependencies]
tetrio-api = { "git" = "https://github.com/Takathediscordbot/tetrio-api", rev="64a0516" }
```

### How to use

###

Fetching a user

```rs
use std::f64::NAN;

use tetrio_api::{http::client, models::packet::Packet};

#[tokio::main]
async fn main() {
    let user = client::fetch_user_info("takathedinosaur").await.expect("Couldn't fetch the CH.TETR.IO API to find the error! This could have been an error while parsing the data or while trying to send the HTTP request.");

    match user {
        Packet { data: Some(data), .. } => {

            println!("{} (id: {}): {}pps, {}apm {}vs", data.user.username, data.user.id, data.user.league.pps.unwrap_or(NAN), data.user.league.apm.unwrap_or(NAN), data.user.league.vs.unwrap_or(NAN));
        },
        Packet { success, error, .. } => {
            if success {
                eprintln!("The API didn't return an error but no data was found somehow!"); 
                // According to the API documentation that is not a possible case.
                unreachable!();
            }

            eprintln!("An error has occured while trying to fetch the user! {:?}", error)            
        }
    };
}
```

### Examples

There are code examples in the (https://github.com/Takathediscordbot/tetrio-api/tree/main/examples)[examples folder] of this git repository. 
The readme in there will indicate how to run examples.

### Fetching a User


### Running tests

To make sure the library functionalities are working before using it, you can run tests by running

```bash
cargo test
```

However, there is a good chance running the tests fail because of rate limit,
you can limit the number of threads running at the same time by using 

```bash
cargo test -- --test-threads=2
```

The tests will run significantly slower but should not fail due to rate limit. 
You might also be able to put a higher number, and if the tests still don't work because of rate limit
you can limit the threads to 1 to remove parallelism completely.




