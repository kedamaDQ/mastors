use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Status,
    methods::{
        Method,
        MethodInternal,
    },
};

pub fn post(conn: &Connection, id: impl Into<String>) -> PostFavourite {
    PostFavourite {
        conn,
        id: id.into(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostFavourite<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    #[serde(skip_serializing)]
    id: String,
}

impl<'a> Method<'a, Status> for PostFavourite<'a> {
    fn send(&self) -> Result<Status> {
        Ok(self.post()?)
    }
}

impl<'a> MethodInternal<'a, Status> for PostFavourite<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses/__id__/favourite";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn path(&self) -> String {
        Self::ENDPOINT.replace("__id__", self.id.as_str())
    }

    fn authorization(&self) -> Option<&str> {
        Some(self.conn.access_token())
    }
}
