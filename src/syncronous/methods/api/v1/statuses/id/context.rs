use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Context,
    methods::{
        Method,
        MethodInternal,
    }
};

pub fn get(conn: &Connection, id: impl Into<String>) -> GetContext {
    GetContext {
        conn,
        id: id.into(),
        authorized: false,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetContext<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    #[serde(skip_serializing)]
    id: String,
    #[serde(skip_serializing)]
    authorized: bool,
}

impl<'a> GetContext<'a> {
    pub fn authorized(&mut self) -> &Self {
        self.authorized = true;
        self
    }

    pub fn unauthorized(&mut self) -> &Self {
        self.authorized = false;
        self
    }
}

impl<'a> Method<'a, Context> for GetContext<'a> {
    fn send(&self) -> Result<Context> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Context> for GetContext<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses/__id__/context";

    fn connection(&self) -> &Connection {
        &self.conn
    }

    fn path(&self) -> String {
        Self::ENDPOINT.replace("__id__", self.id.as_str())
    }

    fn authorization(&self) -> Option<&str> {
        if self.authorized {
            Some(self.conn.access_token())
        } else {
            None
        }
    }

}
