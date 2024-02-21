/// This module provides features related to handling reports for moderators
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::{
    admin::report::Report,
    }
};

pub fn get(conn: &Connection, id: impl Into<String>) -> GetReport {
    GetReport {
        conn,
        authorized: true,
        id: id.into(),
    }
}

#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Report, "/api/v1/admin/reports/_PATH_PARAM_")]
pub struct GetReport<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,
}

impl<'a> Method<'a, Report> for GetReport<'a> {}