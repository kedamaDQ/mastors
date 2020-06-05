# Mastors

The `mastors` crate provides client API for Mastodon.

Currently, mastors is not in relase version. Add mastors to your Cargo.toml with github URL.

```toml
[dependencies]
mastors = { git = "https://github.com/kedamaDQ/mastors", branch = "master" }
```

## REST API

The structure of the Mastors module is consistent with the REST API path on the Mastodon server.

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use mastors::Method;

    let conn = mastors::Connection::new_with_path(".env")?;

    let instance = mastors::api::v1::instance::get(&conn).send()?;

    println!("{:#?}", instance);

    let posted_status = mastors::api::v1::statuses::post(&conn, "Toot!")
        .spoiler_text("Spoiler!")
        .unlisted()
        .send()?
        .status()
        .unwrap();

    println!("{:#?}", posted_status);

    let got_status = mastors::api::v1::statuses::id::get(&conn, posted_status.id())
        .send()?;

    assert_eq!(posted_status.id(), got_status.id());

    let deleted_status = mastors::api::v1::statuses::id::get(&conn, got_status.id())
        .send()?;

    assert_eq!(got_status.id(), deleted_status.id());

    Ok(())
}
```

## Streaming API

Mastors provides streaming timeline with server-sent events as `Iterator`.

```rust
//! This is a simple streaming timeline on the command-line terminal.
use mastors::Method;
use mastors::api::v1::streaming::{
    EventType,
    StreamType,
    get,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error> {
    let conn = mastors::Connection::new_with_path(".env")?;
    let home_timeline = get(&conn, StreamType::User).send()?;

    for event in home_timeline {
        if let EventType::Update(status) = event? {
            println!(
                "{}\n\n{} Posted by {}",
                status.content()?, // As HTML
                status.created_at(),
                status.account().username(),
            );
        }
    }

    Ok(())
}
```

# Connection settings

Mastors loads the connection settings from file that is named ".env" in the current working directory by default.
Connection setting requires `SERVER_URL` and `ACCESS_TOKEN` at least.

```bash
SERVER_URL="http://localhost:3000"
ACCESS_TOKEN="aabbcc"
```

See `Connection` for other optional settings.

# Run tests

Currently, a series of tests will send too many requests to the server.
Only run the test against your own server or a server that is allowed to do it.

Also, currently, a series of tests must be run serialized.

```
cargo test -- --test-threads=1
```

