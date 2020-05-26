use serde::Serialize;
use crate::{
    Connection,
    entities::Instance,
    methods::Method,
};

pub fn get(conn: &Connection) -> GetInstance {
    GetInstance {
        conn,
        authorization: conn.whitelist_mode(),
    }
}

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

pub mod peers {
    use serde::Serialize;
    use crate::{
        Connection,
        entities::instance::Peers,
        methods::Method,
    };

    pub fn get(conn: &Connection) -> GetPeers {
        GetPeers {
            conn
        }
    }

    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Peers, "/api/v1/instance/peers")]
    pub struct GetPeers<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    }

    impl<'a> Method<'a, Peers> for GetPeers<'a> {}
}

pub mod activity {
    use serde::Serialize;
    use crate::{
        Connection,
        entities::Activities,
        methods::Method,
    };

    pub fn get(conn: &Connection) -> GetActivity {
        GetActivity {
            conn
        }
    }

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
    const ENV_TEST: &str = ".env.test";

    #[test]
    fn test_get_instance() {
        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        get(&conn).send().unwrap();
    }

    #[test]
    fn test_get_peers() {
        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        peers::get(&conn).send().unwrap();
    }

    #[test]
    fn test_get_activity() {
        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        activity::get(&conn).send().unwrap();
    }
}
