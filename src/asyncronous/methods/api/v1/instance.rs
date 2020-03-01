use async_trait::async_trait;
use serde::Serialize;
use crate::{
    Connection,
    Result,
    entities::Instance,
    methods::Method,
};

pub fn get(conn: &Connection) -> GetInstance {
    GetInstance {
        conn,
    }
}

#[derive(Serialize)]
pub struct GetInstance<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
}

impl<'a> GetInstance<'a> {}

#[async_trait]
impl<'a> Method<'a, Instance> for GetInstance<'a> {
    const ENDPOINT: &'a str = "/api/v1/instance";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization_code(&self) -> Option<&str> {
        if self.conn.whitelist_mode() {
            Some(self.conn.access_token())
        } else {
            None
        }
    }

    async fn send(&'a self) -> Result<Instance> {
        Ok(self.get().await?)
    }
}

pub mod peers {
    use async_trait::async_trait;
    use serde::Serialize;
    use crate::{
        Connection,
        Result,
        entities::instance::Peers,
        methods::Method,

    };

    #[derive(Serialize, Debug)]
    pub struct GetPeers<'a> {
        #[serde(skip_serializing)]
        conn: &'a Connection,
    }

    #[async_trait]
    impl<'a> Method<'a, Peers> for GetPeers<'a> {
        const ENDPOINT: &'a str = "/api/v1/instance/peers";

        fn connection(&self) -> &Connection {
            self.conn
        }

        fn authorization_code(&self) -> Option<&str> {
            None
        }

        async fn send(&'a self) -> Result<Peers> {
            Ok(self.get().await?)
        }
    }

    pub fn get(conn: &Connection) -> GetPeers {
        GetPeers {
            conn
        }
    }
}

pub mod activity {
    use async_trait::async_trait;
    use serde::Serialize;
    use crate::{
        Connection,
        Result,
        entities::Activities,
        methods::Method,
    };

    #[derive(Debug, Serialize)]
    pub struct GetActivity<'a> {
        #[serde(skip_serializing)]
        conn: &'a Connection,
    }

    #[async_trait]
    impl<'a> Method<'a, Activities> for GetActivity<'a> {
        const ENDPOINT: &'a str = "/api/v1/instance/activity";

        fn connection(&self) -> &Connection {
            self.conn
        }

        fn authorization_code(&self) -> Option<&str> {
            None
        }

        async fn send(&'a self) -> Result<Activities> {
            Ok(self.get().await?)
        }
    }

    pub fn get(conn: &Connection) -> GetActivity {
        GetActivity {
            conn
        }
    }
}
