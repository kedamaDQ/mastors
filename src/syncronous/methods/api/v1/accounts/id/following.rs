//! This module provides features related to accounts that the account specified by ID is following.
use serde::Serialize;
use crate::{
    Connection,
    MethodWithRespHeader as Method,
    entities::Accounts,
};

/// Get a request to get following of an account specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetAccountFollowing {
    GetAccountFollowing {
        conn,
        id: id.into(),
        authorized: true,
        max_id: None,
        since_id: None,
        limit: None,
    }
}

/// GET request for `/api/v1/accounts/:id/following`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Accounts, "/api/v1/accounts/_PATH_PARAM_/following", "Link")]
pub struct GetAccountFollowing<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    max_id: Option<String>,
    since_id: Option<String>,
    limit: Option<usize>,
}

impl<'a> GetAccountFollowing<'a> {
    /// Set the max ID of the following accounts to get.
    pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    /// Set the since ID of the following accounts to get.
    pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    /// Set a number of the following accounts to get.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Method<'a, Accounts> for GetAccountFollowing<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_accounts_following() {
        let conn = Connection::new().unwrap();
        let _got = get(&conn, "1")
            .limit(1)
            .send()
            .unwrap();
    }
}
