//! This module provides features related to get streaming timelines.
use reqwest::{
    header,
    blocking::Client as ReqwestClient,
};
use crate::{
    Connection,
    Error,
    Result,
    streaming::*,
    utils,
};

/// Get the event stream of the type specified by `stream_type`.
pub fn get(conn: &Connection, stream_type: StreamType) -> GetStreaming {
    GetStreaming {
        conn,
        stream_type,
    }
}

/// Get request for `/api/v1/streaming`.
pub struct GetStreaming<'a> {
    conn: &'a Connection,
    stream_type: StreamType,
}

impl<'a> GetStreaming<'a> {
    pub fn send(&self) -> Result<SseStream> {
        let url = self.conn.server_url()
            .join(&self.stream_type.to_string())?;

        let mut headers = header::HeaderMap::new();

        match self.stream_type {
            StreamType::User | StreamType::List(_) | StreamType::Direct => {
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(
                        format!(
                            "Bearer {}", self.conn.access_token()
                        ).as_str()
                    )?
                );
            },
            _ => (),
        };

        let custom_client = ReqwestClient::builder()
            .default_headers(headers)
            .user_agent(self.conn.user_agent())
            .build()
            .map_err(Error::HttpClientError)?;
          
        Ok(SseStream::new(url, custom_client))
    }
}

/// This module provides features related to check about streaming of the server is alives.
pub mod health {
    use super::*;
    use log::trace;

    /// Gets whether the server's streaming is alive.
    pub fn get(conn: &Connection) -> GetHealth {
        GetHealth {
            conn
        }       
    }

    /// GET request for `/api/v1/streaming/health`.
    pub struct GetHealth<'a> {
        conn: &'a Connection,
    }

    impl<'a> GetHealth<'a> {
        const ENDPOINT: &'a str = "/api/v1/streaming/health";

        /// If streaming of the server is alive, will returns a text 'OK'.
        pub fn send(&self) -> Result<String>{
            let req = self.conn.client().get(self.conn.url(Self::ENDPOINT)?).build()?;
            trace!("Send a {} request to {}", req.method(), req.url());

            let res = self.conn.client().execute(req)?;
            trace!("{:?}", res);

            Ok(utils::check_response(res)?.text()?)
        }
    }
}

#[cfg(test)]
mod tests {

}
