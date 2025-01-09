# tetrio-api

[![codecov](https://codecov.io/gh/Takathediscordbot/tetrio-api/graph/badge.svg?token=NPGQ45PP4B)](https://codecov.io/gh/Takathediscordbot/tetrio-api)

A library to interact with the [TETR.IO public API](https://ch.tetr.io), with both a simple http client that doesn't manage cache and a "cached-http-client" that holds the data fetched from the API in memory. 
A library already exists to interact with the API in rust [here](https://github.com/Rinrin0413/tetr-ch-rs),but I still needed an easier way to interact with the cache so I had to work on this library in the end.

The library tries to stay accurate to the [API specs](https://tetr.io/about/api/) but I had to take some liberties because: 
- I needed to access replay data, which are not documented
- Sometimes the documentation was partly innacurate which would cause errors when trying to use the library.

This library **might not** be perfect, but I did write tests, and since rust models are strict, the models have to comply with the current state of the API for the tests to pass, making it essentially accurate and usable.

**THIS IS NOT AN OFFICIAL TETR.IO AFFILIATED LIBRARY.**

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

### Examples

There are code examples in the [examples folder](https://github.com/Takathediscordbot/tetrio-api/tree/main/examples) of this git repository. 
The readme in there will indicate how to run examples.

### Fetching a User


### Running tests

To make sure the library functionalities are working before using it, you can run tests by running

```bash
cargo test
```


