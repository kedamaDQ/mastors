//! This module provides features related to followers of an account specified by ID.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    Result,
    entities::Accounts,
    entities::PaginatedAccounts,
};

/// Get a request to get followers of an account specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetAccountFollowers {
    GetAccountFollowers {
        conn,
        id: id.into(),
        authorized: true,
        max_id: None,
        since_id: None,
        limit: None,
    }
}

/// GET request for `/api/v1/accounts/:id/followers`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, PaginatedAccounts, "/api/v1/accounts/_PATH_PARAM_/followers")]
pub struct GetAccountFollowers<'a> {
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

impl<'a> GetAccountFollowers<'a> {
    /// Set the max ID of the follower accounts to get.
    pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    /// Set the since ID of the follower accounts to get.
    pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    /// Set a number of the follower accounts to get.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Method<'a, PaginatedAccounts> for GetAccountFollowers<'a> {
    fn send(&self) -> Result<PaginatedAccounts> {
        use crate::methods::private::{
            build_request,
            send_request,
        };

        let resp = send_request(
            build_request(self, reqwest::Method::GET)?.json(&self)
        )?;

        let link = resp.headers().get("Link");
        match link {
            Some(link) => {
                Ok(
                    PaginatedAccounts(
                        link.to_str()?.to_owned(),
                        resp.json::<Accounts>()?
                    )
                )
            },
            None => {
                panic!("It is unexpected that the HTTP response header \"Link\" does not exist in response from `/api/v1/accounts/:id/followers`.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_accounts_followers() {
        let conn = Connection::new().unwrap();
        let _got = get(&conn, "1")
            .limit(1)
            .send()
            .unwrap();
    }
}
