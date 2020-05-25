pub mod id;

use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::ScheduledStatuses,
    methods::Method,
};


pub fn get(conn: &Connection) -> GetScheduledStatuses {
    GetScheduledStatuses {
        conn,
        auth: true,
        limit: None,
        max_id: None,
        since_id: None,
        min_id: None,
    }
}

#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, ScheduledStatuses, "/api/v1/scheduled_statuses")]
pub struct GetScheduledStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    limit: Option<usize>,
    max_id: Option<String>,
    since_id: Option<String>,
    min_id: Option<String>,
}

impl<'a> GetScheduledStatuses<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    pub fn min_id(mut self, min_id: impl Into<String>) -> Self {
        self.min_id = Some(min_id.into());
        self
    }
}

impl<'a> Method<'a, ScheduledStatuses> for GetScheduledStatuses<'a> {}
