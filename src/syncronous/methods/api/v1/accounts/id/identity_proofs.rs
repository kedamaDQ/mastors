/// This module provides features related to identity proofs of account specified by ID.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::IdentityProofs,
};

/// Get a request to get identity proofs of account specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetIdentityProofs {
    GetIdentityProofs {
        conn,
        id: id.into(),
        authorized: true,
    }
}

/// GET request for `/api/v1/accounts/_PATH_PARAM_/identity_proofs`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, IdentityProofs, "/api/v1/accounts/_PATH_PARAM_/identity_proofs")]
pub struct GetIdentityProofs<'a> {
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

impl<'a> Method<'a, IdentityProofs> for GetIdentityProofs<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_identity_proofs() {
        let conn = Connection::new().unwrap();
        let _get = get(&conn, "1").send().unwrap();
    }
}
