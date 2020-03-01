use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Trends,
    methods::{
        Method,
        MethodInternal,
    },
};

pub fn get(conn: &Connection) -> GetTrends {
    GetTrends { conn, limit: None, }
}

#[derive(Debug, Serialize)]
pub struct GetTrends<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    limit: Option<u32>,
}

impl<'a> GetTrends<'a> {
    pub fn limit(&mut self, limit: u32) -> &Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Method<'a, Trends> for GetTrends<'a> {
    fn send(&'a self) -> Result<Trends> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Trends> for GetTrends<'a> {
    const ENDPOINT: &'a str = "/api/v1/trends";

    fn connection(&self) -> &Connection {
        self.conn
    }
}

#[cfg(test)]
mod tests {
}
