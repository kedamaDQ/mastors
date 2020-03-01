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

pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnfavourite {
    PostUnfavourite {
        conn,
        id: id.into(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostUnfavourite<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    #[serde(skip_serializing)]
    id: String,
}

impl<'a> Method<'a, Status> for PostUnfavourite<'a> {
    fn send(&self) -> Result<Status> {
        Ok(self.post()?)
    }
}

impl<'a> MethodInternal<'a, Status> for PostUnfavourite<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses/__id__/unfavourite";

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
