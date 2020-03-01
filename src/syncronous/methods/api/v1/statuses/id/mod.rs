pub mod bookmark;
pub mod context;
pub mod favourite;
pub mod favourited_by;
pub mod mute;
pub mod pin;
pub mod reblog;
pub mod reblogged_by;
pub mod unbookmark;
pub mod unfavourite;
pub mod unmute;
pub mod unpin;
pub mod unreblog;

use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::{
        Status,
    },
    methods::{
        Method,
        MethodInternal,
    },
};

/// Create a request to get a status specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetStatuses {
    GetStatuses {
        conn,
        id: id.into(),
        authorized: true,
    }
}

/// GET request for /api/v1/statuses/:id
#[derive(Debug, Serialize)]
pub struct GetStatuses<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    authorized: bool,

    #[serde(skip_serializing)]
    id: String,
}

impl<'a> GetStatuses<'a> {
    /// Add Authorization header to GET request.
    pub fn authorized(&mut self) -> &Self {
        self.authorized = true;
        self
    }

    /// Remove Authorization header from GET request.
    pub fn unauthorized(&mut self) -> &Self {
        self.authorized = false;
        self
    }
}

impl<'a> Method<'a, Status> for GetStatuses<'a> {
    fn send(&'a self) -> Result<Status> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Status> for GetStatuses<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn path(&self) -> String {
        format!("{}/{}", Self::ENDPOINT, &self.id)
    }

    fn authorization(&self) -> Option<&str> {
        if self.authorized {
            Some(self.conn.access_token())
        } else {
            None
        }
    }
}

pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteStatuses {
    DeleteStatuses {
        conn,
        id: id.into(),
    }
}

/// DELETE request for /api/v1/statuses/:id
#[derive(Debug, Serialize)]
pub struct DeleteStatuses<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    #[serde(skip_serializing)]
    id: String,
}

impl<'a> Method<'a, Status> for DeleteStatuses<'a> {
    fn send(&self) -> Result<Status> {
        Ok(self.delete()?)
    }
}

impl<'a> MethodInternal<'a, Status> for DeleteStatuses<'a> {
    const ENDPOINT: &'a str = "/api/vi/statuses";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn path(&self) -> String {
        format!("{}/{}", Self::ENDPOINT, self.id)
    }

    fn authorization(&self) -> Option<&str> {
        Some(self.conn.access_token())
    }
}

