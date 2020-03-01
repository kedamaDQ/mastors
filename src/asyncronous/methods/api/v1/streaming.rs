use async_tungstenite::{
    tokio::connect_async,
    tungstenite::Message,
    tungstenite::handshake::client::Request,
};
use futures::prelude::*;
use serde::Deserialize;
use serde_json;
use crate::{
    Connection,
    Error,
    Result,
    api::v1::instance,
    methods::Method,
};
use std::fmt;

const ENDPOINT: &str = "/api/v1/streaming";

pub async fn get(conn: &Connection, stream: StreamType) -> Result<Stream> {

    let url = instance::get(conn)
        .send()
        .await?
        .urls()
        .streaming_api()
        .join(format!("{}?{}", ENDPOINT, stream).as_str())
        .expect("Failed to create a streaming api endpoint");

    let req = Request::builder()
        .uri(url.as_str())
        .header("Authorization", format!("Bearer {}", conn.access_token()))
        .method("GET")
        .body(())
        .expect("Failed to create a http request");

    let (stream, _) = connect_async(req).await.unwrap();

    Ok(Stream{
        stream
    })
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

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamType::User => write!(f, "stream=user"),
            StreamType::Public => write!(f, "stream=public"),
            StreamType::PublicLocal => write!(f, "stream=public:local"),
            StreamType::Hashtag(tag) => write!(f, "stream=hashtag&tag={}", tag),
            StreamType::HashtagLocal(tag) => write!(f, "stream=hashtag:local&tag={}", tag),
            StreamType::List(id) => write!(f, "stream=list&id={}", id),
            StreamType::Direct => write!(f, "stream=direct"),
        }
    }
}


use async_tungstenite::{
    WebSocketStream,
    stream::Stream as TungStream,
    tokio::TokioAdapter,
};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;

type TcpStr = TokioAdapter<TcpStream>;
type TlsStr = TokioAdapter<TlsStream<TokioAdapter<TcpStr>>>;

pub struct Stream {
    stream: WebSocketStream<TungStream<TcpStr, TlsStr>>,
}

impl Stream {
    pub async fn next_msg(&mut self) -> Result<Option<EventType>> {
        match self.stream.next().await {
            Some(result) => match result {
                Ok(message) => match message {
                    Message::Text(t) => {
                        Ok(Some(get_event_type(&t)?))
                    },
                    Message::Ping(d) => {
                        self.stream.send(Message::Pong(d.clone())).await?;
                        Ok(Some(EventType::Ping(d)))
                    },
                    _ => Ok(Some(EventType::Unknown)),
                },
                Err(e) => Err(Error::WebSocketError(e)),
            },
            None => Ok(None),
        }
    }
}

fn get_event_type(s: &str) -> Result<EventType> {
    let data = serde_json::from_str::<Event>(s)?;

    match data.event.as_str() {
        "update" => Ok(EventType::Update(
            Box::new(serde_json::from_str::<Status>(&data.payload)?)
        )),
        "notification" => Ok(EventType::Notification(
            Box::new(serde_json::from_str::<Notification>(&data.payload)?)
        )),
        "delete" => Ok(EventType::Delete(data.payload.to_owned())),
        "filters_changed" => Ok(EventType::FiltersChanged),
        _ => Err(Error::UnknownEventTypeError(data.event.to_owned()))
    }
}

use crate::entities::{
    Notification,
    Status,
};

#[derive(Debug)]
pub enum EventType {
    Update(Box<Status>),
    Notification(Box<Notification>),
    Delete(String),
    FiltersChanged,
    Ping(Vec<u8>),
    Unknown,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
struct Event {
    event: String,
    payload: String,
}
