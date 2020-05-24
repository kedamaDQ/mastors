use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Trends,
    methods::Method,
};

pub fn get(conn: &Connection) -> GetTrends {
    GetTrends {
        conn,
        limit: None,
    }
}

#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(GET, Trends, "/api/v1/trends")]
pub struct GetTrends<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    limit: Option<usize>,
}

impl<'a> GetTrends<'a> {
    pub fn limit(&mut self, limit: usize) -> &Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Method<'a, Trends> for GetTrends<'a> {}
