use async_trait::async_trait;
use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Trends,
    methods::Method,
};

pub fn get(conn: &Connection) -> GetTrend {
    GetTrend { conn, limit: None, }
}

#[derive(Debug, Serialize)]
pub struct GetTrend<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    limit: Option<u32>,
}

impl<'a> GetTrend<'a> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[async_trait]
impl<'a> Method<'a, Trends> for GetTrend<'a> {
    const ENDPOINT: &'a str = "/api/v1/trends";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization_code(&self) -> Option<&str> {
        None
    }
    
    async fn send(&'a self) -> Result<Trends> {
        Ok(self.get().await?)
    }
}

#[cfg(test)]
mod tests {
}
