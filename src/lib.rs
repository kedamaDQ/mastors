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
//! // Display toot that is you posted and returned from the server.
//! println!("{:#?}", posted_status);
//! #
//! # let _deleted = mastors::api::v1::statuses::id::delete(&conn, posted_status.id()).send()?;
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
//! let posted_status = mastors::api::v1::statuses::post(&conn)
//!     .status("Toot!")
//!     .spoiler_text("Spoiler!")
//!     .unlisted()
//!     .send()?;
//! println!("{:#?}", posted_status);
//! 
//! // Get a toot that posted in the previous step.
//! let got_status = mastors::api::v1::statuses::id::get(&conn, posted_status.id())
//!     .send()?;
//! assert_eq!(posted_status.id(), got_status.id());
//! 
//! // Delete a toot.
//! let deleted_status = mastors::api::v1::statuses::id::delete(&conn, got_status.id())
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
//! ## Connection settings
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
//! ## Run tests
//! 
//! **HIGHLY RECOMMENDED**: If you run tests, please run tests on your local server, which is localhost:3000.
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
// This import currently unused, used by scope.
//#[macro_use] extern crate lazy_static;

mod connection;
mod error;
mod synchronous;
mod utils;

pub mod entities;
//pub mod scope;

pub use connection::Connection;
pub use error::{ Error, Result };

pub use chrono::DateTime;
pub use chrono::Utc;
pub use url::Url;

// Use to switch to asynchronous mode if ready in the future...
use synchronous as current_mode;

pub use current_mode::{
    methods::{
        Method,
        MethodWithRespHeader,
        api,
    },
    streaming,
};

use current_mode::methods::private;

/// This module provides some convenient functions and exports to use mastors.
pub mod prelude {
    pub use chrono::DateTime;
    pub use chrono::Utc;
    pub use url::Url;

    pub use crate::{
        Connection,
        Method,
        MethodWithRespHeader,
    };
    pub use crate::entities::*;
    pub use crate::streaming::*;

    /// Toot a simple text.
    pub fn toot(conn: &Connection, body: impl AsRef<str>) -> crate::Result<crate::entities::Status> {
        crate::api::v1::statuses::post(conn).status(body).send()
    }

    /// Get your home timeline stream.
    pub fn home_timeline(conn: &Connection) -> crate::Result<impl StreamingTimeline> {
        crate::api::v1::streaming::get(conn, StreamType::User).send()
    }

    /// Get the local timeline stream.
    pub fn local_timeline(conn: &Connection) -> crate::Result<impl StreamingTimeline> {
        crate::api::v1::streaming::get(conn, StreamType::PublicLocal).send()
    }

    /// Get the public timeline stream.
    pub fn public_timeline(conn: &Connection) -> crate::Result<impl StreamingTimeline> {
        crate::api::v1::streaming::get(conn, StreamType::Public).send()
    }

    /// Get the hashtag timeline stream.
    pub fn hashtag_timeline(conn: &Connection, tag: impl Into<String>) -> crate::Result<impl StreamingTimeline> {
        crate::api::v1::streaming::get(conn, StreamType::Hashtag(tag.into())).send()
    }
}
