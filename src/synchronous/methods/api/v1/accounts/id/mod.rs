//! This module provides features related to account specified by ID.

mod blocks;
pub use blocks::{
    block as block,
    unblock as unblock,
};
mod follows;
pub use follows::{
    followers,
    following,
    follow,
    unfollow,
};
pub mod identity_proofs;
pub mod lists;
mod mutes;
pub use mutes::{
    mute,
    unmute,
};
mod pins;
pub use pins::{
    pin,
    unpin,
};
pub mod statuses;

use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Account,
};

/// Get a request to get account specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetAccount {
    GetAccount {
        conn,
        id: id.into(),
        authorized: conn.whitelist_mode()
    }
}

/// GET request for `/api/v1/accounts/:id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Account, "/api/v1/accounts/_PATH_PARAM_")]
pub struct GetAccount<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,
}

impl<'a> Method<'a, Account> for GetAccount<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account() {
        let conn = Connection::new().unwrap();

        get(&conn, id()).send().unwrap();
    }

    use crate::api::v1::accounts::verify_credentials;
    fn id() -> String {
        let conn = Connection::new().unwrap();
        verify_credentials::get(&conn).send().unwrap().id().to_owned()
    }
}
