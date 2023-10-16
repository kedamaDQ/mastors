//! This module provides features related to lists that the account specified by ID is owning.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Lists,
};

/// Get a request to get lists that the account specified by `id` is owning.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetLists {
    GetLists {
        conn,
        id: id.into(),
        authorized: true
    }
}

/// GET request for `/api/v1/accounts/:id/lists`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Lists, "/api/v1/accounts/_PATH_PARAM_/lists")]
pub struct GetLists<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,
}

impl<'a> Method<'a, Lists> for GetLists<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lists() {
        let conn = Connection::new().unwrap();
        let _get = get(&conn, id()).send().unwrap();
    }

    use crate::api::v1::accounts::verify_credentials;
    fn id() -> String {
        let conn = Connection::new().unwrap();
        verify_credentials::get(&conn).send().unwrap().id().to_owned()
    }
}
