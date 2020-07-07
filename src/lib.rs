//! # Mastors
//! 
//! The `mastors` crate provides client API for Mastodon.
//! 
//! This is an example of simply posting a toot.
//! 
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use mastors::prelude::*;
//! 
//! # // cfg(test) is not set during doctests
//! # // https://github.com/rust-lang/rust/issues/45599
//! let conn = Connection::from_file(".env.test")?;
//! let posted_status = toot(&conn, "Toot!")?;
//! 
//! // Display toot that is you posted and returned fron the server.
//! println!("{:#?}", posted_status);
//! # Ok(())
//! # }
//! ```
//! 
//! ## REST API
//! 
//! The structure of the Mastors module is consistent with the REST API path on the Mastodon server.
//! 
//! ```rust
//! # use std::result::Result as StdResult;
//! # use std::error::Error as StdError;
//! #
//! # fn main() -> StdResult<(), Box<dyn StdError>> {
//! use mastors::prelude::*;
//! 
//! # // cfg(test) is not set during doctests
//! # // https://github.com/rust-lang/rust/issues/45599
//! let conn = Connection::from_file(".env.test")?;
//! 
//! // Just get the server information from `/api/v1/instance` endpoint simply.
//! let instance = mastors::api::v1::instance::get(&conn).send()?;
//! println!("{:#?}", instance);
//! 
//! // Post a toot with spoiler text and unlisted visibility.
//! let posted_status = mastors::api::v1::statuses::post(&conn, "Toot!")
//!     .spoiler_text("Spoiler!")
//!     .unlisted()
//!     .send()?
//!     .status()
//!     .unwrap();
//! println!("{:#?}", posted_status);
//! 
//! // Get a toot that posted in the previous step.
//! let got_status = mastors::api::v1::statuses::id::get(&conn, posted_status.id())
//!     .send()?;
//! assert_eq!(posted_status.id(), got_status.id());
//! 
//! // Delete a toot.
//! let deleted_status = mastors::api::v1::statuses::id::get(&conn, got_status.id())
//!     .send()?;
//! assert_eq!(got_status.id(), deleted_status.id());
//! # Ok(())
//! # }
//! ```
//! 
//! ## Streaming API
//! 
//! Mastors provides streaming timeline with server-sent events as `Iterator`.
//! 
//! ```no_run
//! //! This is a simple streaming timeline on the command-line terminal.
//! # use std::result::Result as StdResult;
//! # use std::error::Error as StdError;
//! #
//! # fn main() -> StdResult<(), Box<dyn StdError>> {
//! 
//! use mastors::prelude::*;
//! 
//! # // cfg(test) is not set during doctests
//! # // https://github.com/rust-lang/rust/issues/45599
//! let conn = Connection::from_file(".env.test")?;
//! let home_timeline = home_timeline(&conn)?;
//! 
//! for event in home_timeline {
//!     if let EventType::Update(status) = event? {
//!         println!(
//!             "{}\n\n{} Posted by {}",
//!             status.content().unwrap(), // As HTML
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
//! In order to run the test, you need to prepare the connection settings in file `.env.test`.
//! 
//! Currently, a series of tests will send too many requests to the server.
//! Only run the test against your own server or a server that is allowed to do it.
//! 
//! Also, currently, a series of tests must be run serialized.
//! 
//! ```bash
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
        MethodWithRespHeader,
        api,
        self,
    },
    streaming_timeline,
};

/// This module provides some convenient functions and exports to use mastors.
pub mod prelude {
    pub use chrono::DateTime;
    pub use chrono::Utc;
    pub use url::Url;

    pub use crate::{
        Connection,
        Error,
        Method,
        MethodWithRespHeader,
        Result,
    };
    pub use crate::streaming_timeline::*;

    /// Toot a simple text.
    pub fn toot(conn: &Connection, body: impl AsRef<str>) -> Result<Box<crate::entities::Status>> {
        match crate::api::v1::statuses::post(conn, body).send() {
            Ok(posted) => Ok(posted.status().unwrap()),
            Err(e) => Err(e),
        }
    }

    /// Get your home timeline stream.
    pub fn home_timeline(conn: &Connection) -> Result<SseStream> {
        crate::api::v1::streaming::get(conn, StreamType::User).send()
    }

    /// Get the local timeline stream.
    pub fn local_timeline(conn: &Connection) -> Result<SseStream> {
        crate::api::v1::streaming::get(conn, StreamType::PublicLocal).send()
    }

    /// Get the public timeline stream.
    pub fn public_timeline(conn: &Connection) -> Result<SseStream> {
        crate::api::v1::streaming::get(conn, StreamType::Public).send()
    }

    /// Get the hashtag timeline stream.
    pub fn hashtag_timeline(conn: &Connection, tag: impl Into<String>) -> Result<SseStream> {
        crate::api::v1::streaming::get(conn, StreamType::Hashtag(tag.into())).send()
    }
}
