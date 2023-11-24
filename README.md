# tetrio-api

A library to interact with the [https://ch.tetr.io](TETR.IO public API), with both a simple http client that doesn't manage cache and a "cached-http-client" that holds the data fetched from the API in memory. 
A library already exists to interact with the API in rust (https://github.com/Rinrin0413/tetr-ch-rs)[here], it is easier to use because it has been published on crates.io and stuff, but I still needed an easier way to interact with the cache so I had to work on this library in the end.

The library tries to stay accurate to the [https://tetr.io/about/api/](API specs) but I had to take some liberties because: 
- I needed to access replay data, which are not documented
- Sometimes the documentation was partly innacurate which would cause errors when trying to use the library.

This library **might not** be perfect, but I did write tests, and since rust models are strict, the models have to comply with the current state of the API for the tests to pass, making it essentially accurate and usable.

## Quick start

### Install

Simply add this to your Cargo.toml

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

### Examples

There are code examples in the examples folder of this git repository. 
The readme in there will indicate how to run examples.

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




