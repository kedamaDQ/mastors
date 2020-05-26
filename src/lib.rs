//! mastors
//! 
//! The `mastors` crate provides client API for Mastodon.
//! 
//! ```rust
//!     use mastors::Method;
//! 
//!     let conn = mastors::Connection::new_with_path(".env").unwrap();
//! 
//!     let instance = mastors::api::v1::instance::get(&conn).send().unwrap();
//! 
//!     println!("{:#?}", instance);
//! 
//!     let status = mastors::api::v1::statuses::post(&conn, "Toot!")
//!         .spoiler_text("Spoiler!")
//!         .unlisted()
//!         .send()
//!         .unwrap();
//! 
//!     println!("{:#?}", status);
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

pub use chrono::{ DateTime, Duration, Local, Utc };
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
