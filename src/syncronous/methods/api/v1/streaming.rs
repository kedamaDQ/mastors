use eventsource::{
    event::Event,
    reqwest::Client,
};
use reqwest::{
    header,
    blocking::Client as ReqwestClient,
};
use serde_json;
use crate::{
    Connection,
    Error,
    Result,
    entities::{
        Notification,
        Status,
    },
    utils,
};

pub use crate::streaming_timeline::*;

const ENDPOINT: &str = "/api/v1/streaming";

pub fn get(conn: &Connection, stream_type: StreamType) -> GetStreaming {
    GetStreaming {
        conn,
        stream_type,
    }
}

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
            .tcp_nodelay()
            .user_agent(self.conn.user_agent())
            .build()
            .map_err(Error::HttpClientError)?;

        Ok(SseStream{
            client: Client::new_with_client(url, custom_client),
        })
    }
}

pub struct SseStream {
    client: Client,
}

impl StreamingTimeline for SseStream {}

impl Iterator for SseStream {

    type Item = Result<EventType>;

    fn next(&mut self) -> Option<Self::Item> {
        self.client.next().map(|result| {
            match result {
                Ok(event) => get_event_type(&event),
                Err(e) => Err(Error::SseStreamError(e)),
            }
        })
    }
}

fn get_event_type(event: &Event) -> Result<EventType> {
    if let Some(event_type) = &event.event_type {
        match event_type.as_str() {
            "update" => {
                Ok(EventType::Update(
                    Box::new(serde_json::from_str::<Status>(&event.data)?)
                ))
            },
            "notification" => {
                Ok(EventType::Notification(
                    Box::new(serde_json::from_str::<Notification>(&event.data)?)
                ))
            },
            "delete" => {
                Ok(EventType::Delete(event.data.to_owned()))
            },
            "filters_changed" => {
                Ok(EventType::FiltersChanged)
            },
            _ => Err(Error::UnknownEventTypeError(event_type.to_owned()))
        }
    } else {
        Ok(EventType::Unknown(event.data.to_owned()))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Clone)]
pub enum StreamType {
    User,
    Public,
    PublicLocal,
    Hashtag(String),
    HashtagLocal(String),
    List(String),
    Direct,
}

use std::fmt;

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamType::User => write!(f, "{}/user", ENDPOINT),
            StreamType::Public => write!(f, "{}/public", ENDPOINT),
            StreamType::PublicLocal => write!(f, "{}/public/local", ENDPOINT),
            StreamType::Hashtag(tag) => write!(f, "{}/hashtag?tag={}", ENDPOINT, tag),
            StreamType::HashtagLocal(tag) => write!(f, "{}/hashtag/local?tag={}", ENDPOINT, tag),
            StreamType::List(id) => write!(f, "{}/list?list={}", ENDPOINT, id),
            StreamType::Direct => write!(f, "{}/direct", ENDPOINT),
        }
    }
}


pub mod health {
    use super::*;

    pub fn get(conn: &Connection) -> GetHealth {
        GetHealth {
            conn
        }       
    }

    pub struct GetHealth<'a> {
        conn: &'a Connection,
    }

    impl<'a> GetHealth<'a> {

        const ENDPOINT: &'a str = "/api/v1/streaming/health";

        pub fn send(&self) -> Result<String>{
            Ok(utils::extract_response(
               self.conn.client().get(self.conn.url(Self::ENDPOINT)?).send()?
            )?.text()?)
        }
    }
}
