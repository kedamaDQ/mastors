use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::{
        Accounts,
    },
    methods::{
        Method,
        MethodInternal,
    },
};

pub fn get(conn: &Connection, id: impl Into<String>) -> GetRebloggedBy {
    GetRebloggedBy {
        conn,
        id: id.into(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRebloggedBy<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    #[serde(skip_serializing)]
    id: String,
}

impl<'a> Method<'a, Accounts> for GetRebloggedBy<'a> {
    fn send(&self) -> Result<Accounts> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Accounts> for GetRebloggedBy<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses/__id__/reblogged_by";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn path(&self) -> String {
        Self::ENDPOINT.replace("__id__", self.id.as_str())
    }

}
