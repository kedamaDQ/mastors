//! mastors
//! 
//! The `mastors` crate provides client API for Mastodon.
//! 
//! ```rust
//!     let conn = mastors::Connection::new_with_path(".env").unwrap();
//!     let instance = mastors::api::v1::instance::get(&conn).send()?;
//!     println!("{:#?}", instance);
//! ```
#[macro_use] extern crate lazy_static;

mod connection;
mod error;
mod syncronous;
mod utils;

pub mod entities;
pub mod scope;

pub use connection::Connection;
pub use error::{ Error, Result };

pub use chrono::{ DateTime, Utc };
pub use isolang::Language;
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
