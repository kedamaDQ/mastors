//! This module provides features related to status specified by ID.
mod bookmarks;
pub use bookmarks::{
    bookmark,
    unbookmark
};
mod contexts;
pub use contexts::{
    context,
};
mod favourites;
pub use favourites::{
    favourited_by,
    favourite,
    unfavourite,
};
mod reblogs;
pub use reblogs::{
    reblogged_by,
    reblog,
    unreblog,
};
mod pins;
pub use pins::{
    pin,
    unpin,
};
mod mutes;
pub use mutes::{
    mute,
    unmute,
};

use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Status,
};

/// Create a request to get a status specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetStatuses {
    GetStatuses {
        conn,
        id: id.into(),
        authorized: true,
    }
}

/// Create a request to delete the status specified by `id`.
pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteStatuses {
    DeleteStatuses {
        conn,
        auth: true,
        id: id.into(),
    }
}

/// GET request for `/api/v1/statuses/:id`.
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(GET, Status, "/api/v1/statuses/_PATH_PARAM_")]
pub struct GetStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,
}

impl<'a> GetStatuses<'a> {
    /// Add Authorization header to GET request.
    pub fn authorized(mut self) -> Self {
        self.authorized = true;
        self
    }

    /// Remove Authorization header from GET request.
    pub fn unauthorized(mut self) -> Self {
        self.authorized = false;
        self
    }
}

impl<'a> Method<'a, Status> for GetStatuses<'a> {}

/// DELETE request for `/api/v1/statuses/:id`.
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(DELETE, Status, "/api/v1/statuses/_PATH_PARAM_")]
pub struct DeleteStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,
}

impl<'a> Method<'a, Status> for DeleteStatuses<'a> {}
