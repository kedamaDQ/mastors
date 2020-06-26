//! This module provides features related to follow the account.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Relationship,
};

/// Get a request to follow an account specified by `id`.
pub fn post(conn: &Connection, id: impl Into<String>) -> PostFollow {
    PostFollow {
        conn,
        id: id.into(),
        authorized: true,
        reblogs: None,
    }
}

/// POST request for `/api/v1/accounts/:id/follow`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/follow")]
pub struct PostFollow<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    // Optional params
    reblogs: Option<bool>,
}

impl<'a> PostFollow<'a> {
    /// Set to show reblogs from account specified by ID.
    pub fn reblogs(mut self) -> Self {
        self.reblogs = Some(true);
        self
    }
}

impl<'a> Method<'a, Relationship> for PostFollow<'a> {}

#[cfg(test)]
mod tests {
    // fmm...
}
