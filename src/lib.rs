//! # Mastors
//! 
//! The `mastors` crate provides client API for Mastodon.
//! 
//! ## REST API
//! 
//! The structure of the Mastors module is consistent with the REST API path on the Mastodon server.
//! 
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use mastors::Method;
//! 
//! # // cfg(test) is not set during doctests
//! # // https://github.com/rust-lang/rust/issues/45599
//! let conn = mastors::Connection::from_file(".env.test")?;
//! 
//! let instance = mastors::api::v1::instance::get(&conn).send()?;
//! 
//! println!("{:#?}", instance);
//! 
//! let posted_status = mastors::api::v1::statuses::post(&conn, "Toot!")
//!     .spoiler_text("Spoiler!")
//!     .unlisted()
//!     .send()?
//!     .status()
//!     .unwrap();
//! 
//! println!("{:#?}", posted_status);
//! 
//! let got_status = mastors::api::v1::statuses::id::get(&conn, posted_status.id())
//!     .send()?;
//! 
//! assert_eq!(posted_status.id(), got_status.id());
//! 
//! let deleted_status = mastors::api::v1::statuses::id::get(&conn, got_status.id())
//!     .send()?;
//! 
//! assert_eq!(got_status.id(), deleted_status.id());
//! #
//! # Ok(())
//! # }
//! ```
//! 
//! ## Streaming API
//! 
//! Mastors provides streaming timeline with server-sent events as `Iterator`.
//! 
//! ```no-run
//! //! This is a simple streaming timeline on the command-line terminal.
//! use mastors::Method;
//! use mastors::api::v1::streaming::{
//!     EventType,
//!     StreamType,
//!     get,
//! };
//!
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error> {
//! 
//! # // cfg(test) is not set during doctests
//! # // https://github.com/rust-lang/rust/issues/45599
//! let conn = mastors::Connection::from_file(".env.test")?;
//! let home_timeline = get(&conn, StreamType::User).send()?;
//! 
//! for event in home_timeline {
//!     if let EventType::Update(status) = event? {
//!         println!(
//!             "{}\n\n{} Posted by {}",
//!             status.content()?, // As HTML
//!             status.created_at(),
//!             status.account().username(),
//!         );
//!     }
//! }
//! #
//! # Ok(())
//! # }
//! ```
//! 
//! # Connection settings
//! 
//! Mastors loads the connection settings from file that is named ".env" in the current working directory by default.
//! Connection setting requires `SERVER_URL` and `ACCESS_TOKEN` at least.
//! 
//! ```bash
//! SERVER_URL="http://localhost:3000"
//! ACCESS_TOKEN="aabbcc"
//! ```
//! 
//! See [`Connection`](struct.Connection.html) for other optional settings.
//! 
//! # Run tests
//! 
//! Currently, a series of tests will send too many requests to the server.
//! Only run the test against your own server or a server that is allowed to do it.
//! 
//! Also, currently, a series of tests must be run serialized.
//! 
//! ```no-run
//! cargo test -- --test-threads=1
//! ```
//! 
#[macro_use] extern crate lazy_static;

mod connection;
mod error;
mod syncronous;
mod utils;

pub mod entities;
pub mod scope;

pub use connection::Connection;
pub use error::{ Error, Result };

pub use chrono::DateTime;
pub use chrono::Utc;
pub use url::Url;

use syncronous as current_mode;

pub use current_mode::{
    methods::{
        Method,
        api,
        self,
    },
    streaming_timeline,
};
