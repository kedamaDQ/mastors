use serde::Serialize;
use crate::{
    Connection,
    DateTime,
    Result,
    Utc,
    entities::ScheduledStatus,
    methods::Method,
};

pub fn get(conn: &Connection, id: impl Into<String>) -> GetScheduledStatuses {
    GetScheduledStatuses {
        conn,
        auth: true,
        id: id.into(),
    }
}

pub fn put(conn: &Connection, id: impl Into<String>) -> PutScheduledStatuses {
    PutScheduledStatuses {
        conn,
        auth: true,
        id: id.into(),
        scheduled_at: None,
    }
}

pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteScheduledStatuses {
    DeleteScheduledStatuses {
        conn,
        auth: true,
        id: id.into(),
    }
}

#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, ScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
pub struct GetScheduledStatuses<'a> {
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

impl<'a> Method<'a, ScheduledStatus> for GetScheduledStatuses<'a> {}

#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, ScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
pub struct PutScheduledStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    scheduled_at: Option<DateTime<Utc>>,
}

impl<'a> PutScheduledStatuses<'a> {
    pub fn scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }
}

impl<'a> Method<'a, ScheduledStatus> for PutScheduledStatuses<'a> {}


#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(DELETE, ScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
pub struct DeleteScheduledStatuses<'a> {
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
