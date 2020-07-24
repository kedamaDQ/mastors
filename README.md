# Mastors

The `mastors` crate provides client API for Mastodon.

This is an example of simply posting a toot.

```rust
use mastors::prelude::*;

let conn = Connection::new()?;
let posted_status = toot(&conn, "Toot!")?;

// Display toot that is you posted and returned from the server.
println!("{:#?}", posted_status);
```

## How to use

Currently, mastors is not in relase version. Add mastors to your Cargo.toml with github URL.

```toml
[dependencies]
mastors = { git = "https://github.com/kedamaDQ/mastors", branch = "master" }
```

## REST API

The structure of the Mastors module is consistent with the REST API path on the Mastodon server.

```rust
use mastors::prelude::*;

let conn = Connection::new()?;

// Just get the server information from `/api/v1/instance` endpoint simply.
let instance = mastors::api::v1::instance::get(&conn).send()?;
println!("{:#?}", instance);

// Post a toot with spoiler text and unlisted visibility.
let posted_status = mastors::api::v1::statuses::post(&conn, "Toot!")
    .spoiler_text("Spoiler!")
    .unlisted()
    .send()?
    .status()
    .unwrap();
println!("{:#?}", posted_status);

// Get a toot that posted in the previous step.
let got_status = mastors::api::v1::statuses::id::get(&conn, posted_status.id())
    .send()?;
assert_eq!(posted_status.id(), got_status.id());

// Delete a toot.
let deleted_status = mastors::api::v1::statuses::id::delete(&conn, got_status.id())
    .send()?;
assert_eq!(got_status.id(), deleted_status.id());
```

## Streaming API

Mastors provides streaming timeline with server-sent events as `Iterator`.

```rust
//! This is a simple streaming timeline on the command-line terminal.
use mastors::prelude::*;

let conn = Connection::new()?;
let home_timeline = home_timeline(&conn)?;

for event in home_timeline {
    if let EventType::Update(status) = event? {
        println!(
            "{}\n\n{} Posted by {}",
            status.content().unwrap(), // As HTML
            status.created_at(),
            status.account().username(),
        );
    }
}
```

## Connection settings

Mastors loads the connection settings from file that is named ".env" in the current working directory by default.
Connection setting requires `SERVER_URL` and `ACCESS_TOKEN` at least.

```bash
SERVER_URL="http://localhost:3000"
ACCESS_TOKEN="aabbcc"
```

See `Connection` for other optional settings.

## Documents

Currently, you have to generate the documentation yourself if you want.

```
cargo doc --no-deps --open
```


## Run tests

In order to run the test, you need to prepare the connection settings in file `.env.test`.

:warning: **HIGHLY RECOMMENDED**: If you run tests, please run tests on your local server, which is localhost:3000.

Currently, a series of tests will send too many requests to the server.
Only run the test against your own server or a server that is allowed to do it.

Also, currently, a series of tests must be run serialized.

```
cargo test -- --test-threads=1
```
