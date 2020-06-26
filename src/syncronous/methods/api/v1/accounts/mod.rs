//! This module provides features related to accounts.
pub mod id;
pub mod relationships;
pub mod search;

use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Account,
};

/// Get a POST request for `/api/v1/accounts`.
#[allow(unused)]
pub fn post(
    conn: &Connection,
    username: impl Into<String>,
    email: impl Into<String>,
    password: impl Into<String>,
    agreement: bool,
    locale: impl Into<String>,
) -> PostAccounts {
    unimplemented!()
}

pub use id::followers::get as get_followers_by_account_id;
pub use id::following::get as get_following_by_account_id;
pub use id::identity_proofs::get as get_identity_proofs_by_account_id;
pub use id::lists::get as get_lists_by_account_id;
pub use id::statuses::get as get_statuses_by_account_id;

/// Post request for `/api/v1/accounts` used to create an account.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Account, "/api/v1/accounts")]
pub struct PostAccounts<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    // Required attributes
    username: String,
    email: String,
    password: String,
    agreement: bool,
    locale: String,

    // Optional attributes
    reason: Option<String>,
}

impl<'a> PostAccounts<'a> {
    /// Set a reason why you want to create an account if server set to manual approval mode.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

impl<'a> Method<'a, Account> for PostAccounts<'a> {}

/// This module provides features to check whether your access token is valid.
pub mod verify_credentials {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Account,
    };

    /// Get a request to check whether your accesstoken is valid and to get your account.
    pub fn get(conn: &Connection) -> GetVerifyCredentials {
        GetVerifyCredentials {
            conn,
            authorized: true,
        }
    }

    /// GET request for `/api/v1/accounts/verify_credentials`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Account, "/api/v1/accounts/verify_credentials")]
    pub struct GetVerifyCredentials<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,

        #[serde(skip_serializing)]
        #[mastors(authorization)]
        authorized: bool,
    }

    impl<'a> Method<'a, Account> for GetVerifyCredentials<'a> {}
}

/// This module provides features related to update your account.
pub mod update_credentials {
    use crate::{
        Connection,
    };

    /// Get request to update your account.
    #[allow(unused)]
    pub fn patch(conn: &Connection) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Connection,
        Method,
    };

    #[test]
    fn test_get_verify_credentials() {
        let conn = Connection::new().unwrap();
        let _account = verify_credentials::get(&conn)
            .send()
            .unwrap();
    }
}
