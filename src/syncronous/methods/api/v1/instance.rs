//! This module provides features related to server information.
use serde::Serialize;
use crate::{
    Connection,
    entities::Instance,
    methods::Method,
};

/// Get a request to get the instance information.
pub fn get(conn: &Connection) -> GetInstance {
    GetInstance {
        conn,
        authorization: conn.whitelist_mode(),
    }
}

/// GET request for `/api/v1/instance`;
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Instance, "/api/v1/instance")]
pub struct GetInstance<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorization: bool,
}

impl<'a> Method<'a, Instance> for GetInstance<'a> {}

/// This module provides features related to the list of the server connected domains.
pub mod peers {
    use serde::Serialize;
    use crate::{
        Connection,
        entities::instance::Peers,
        methods::Method,
    };

    /// Get a request to get the list of the server connected domains.
    pub fn get(conn: &Connection) -> GetPeers {
        GetPeers {
            conn
        }
    }

    /// GET request for `/api/v1/instance/peers`
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Peers, "/api/v1/instance/peers")]
    pub struct GetPeers<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    }

    impl<'a> Method<'a, Peers> for GetPeers<'a> {}
}

/// This module provides features related to weekly activity of the server.
pub mod activity {
    use serde::Serialize;
    use crate::{
        Connection,
        entities::Activities,
        methods::Method,
    };

    /// Get a request to get the server weekly activity.
    pub fn get(conn: &Connection) -> GetActivity {
        GetActivity {
            conn
        }
    }

    /// GET request for `/api/v1/instance/activity`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Activities, "/api/v1/instance/activity")]
    pub struct GetActivity<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    }

    impl<'a> Method<'a, Activities> for GetActivity<'a> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instance() {
        let conn = Connection::new_with_path(crate::ENV_TEST).unwrap();
        get(&conn).send().unwrap();
    }

    #[test]
    fn test_get_peers() {
        let conn = Connection::new_with_path(crate::ENV_TEST).unwrap();
        peers::get(&conn).send().unwrap();
    }

    #[test]
    fn test_get_activity() {
        let conn = Connection::new_with_path(crate::ENV_TEST).unwrap();
        activity::get(&conn).send().unwrap();
    }
}
