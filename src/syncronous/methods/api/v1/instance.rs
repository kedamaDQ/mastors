use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Instance,
    methods::{
        Method,
        MethodInternal,
    }
};

pub fn get(conn: &Connection) -> GetInstance {
    GetInstance {
        conn,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetInstance<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
}

impl<'a> GetInstance<'a> {}

impl<'a> Method<'a, Instance> for GetInstance<'a> {
    fn send(&'a self) -> Result<Instance> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Instance> for GetInstance<'a> {
    const ENDPOINT: &'a str = "/api/v1/instance";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization(&self) -> Option<&str> {
        if self.conn.whitelist_mode() {
            Some(self.conn.access_token())
        } else {
            None
        }
    }
}

pub mod peers {
    use serde::Serialize;
    use crate::{
        Connection,
        Result,
        entities::instance::Peers,
        methods::{
            Method,
            MethodInternal,
        },
    };

    pub fn get(conn: &Connection) -> GetPeers {
        GetPeers {
            conn
        }
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct GetPeers<'a> {
        #[serde(skip_serializing)]
        conn: &'a Connection,
    }

    impl<'a> Method<'a, Peers> for GetPeers<'a> {
        fn send(&'a self) -> Result<Peers> {
            Ok(self.get()?)
        }
    }

    impl<'a> MethodInternal<'a, Peers> for GetPeers<'a> {
        const ENDPOINT: &'a str = "/api/v1/instance/peers";

        fn connection(&self) -> &Connection {
            self.conn
        }
    }
}

pub mod activity {
    use serde::Serialize;
    use crate::{
        Connection,
        Result,
        entities::Activities,
        methods::{
            Method,
            MethodInternal,
        },
    };

    pub fn get(conn: &Connection) -> GetActivity {
        GetActivity {
            conn
        }
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct GetActivity<'a> {
        #[serde(skip_serializing)]
        conn: &'a Connection,
    }

    impl<'a> Method<'a, Activities> for GetActivity<'a> {
        fn send(&'a self) -> Result<Activities> {
            Ok(self.get()?)
        }
    }

    impl<'a> MethodInternal<'a, Activities> for GetActivity<'a> {
        const ENDPOINT: &'a str = "/api/v1/instance/activity";

        fn connection(&self) -> &Connection {
            self.conn
        }
    }
}
