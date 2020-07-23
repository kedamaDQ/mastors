//! This module provides features related to hashtags that are frequently used around the server.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Trends,
};

/// Get a request to get trending hashtags.
pub fn get(conn: &Connection) -> GetTrends {
    GetTrends {
        conn,
        limit: None,
    }
}

/// GET request for `/api/v1/trends`.
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(GET, Trends, "/api/v1/trends")]
pub struct GetTrends<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    limit: Option<usize>,
}

impl<'a> GetTrends<'a> {
    /// Set the limit on the number of hashtags to get.
    pub fn limit(&mut self, limit: usize) -> &Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Method<'a, Trends> for GetTrends<'a> {}

#[cfg(test)]
mod tests {
    // fmm...Local development servers usually don't have trending hashtags.
}
