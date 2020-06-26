use eventsource::{
    event::Event,
    reqwest::Client,
};
use crate::{
    Error,
    Result,
    Url,
    entities::{
        Notification,
        Status,
    },
};

pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {}

/// Represents the stream of each timeline.
pub struct SseStream {
    client: Client,
}

impl SseStream {
    pub fn new(url: Url, client: reqwest::blocking::Client) -> Self {
        SseStream {
            client: Client::new_with_client(url, client),
        }
    }
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

/// Represent the event types of the server-sent events of the Mastodon.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EventType {
    /// A new status has appeared in the timeline.
    /// 
    /// This event type has `Status`.
    Update(Box<Status>),

    /// A new notification has appeared.
    /// 
    /// This event type has `Notification`.
    Notification(Box<Notification>),

    /// A status as been deleted.
    /// 
    /// This event type has ID of the deleted status as `String`.
    Delete(String),

    /// Keyword filters have been changed.
    /// 
    /// This event type has no data.
    FiltersChanged,

    /// This event type contains received raw data as `String`.
    Unknown(String),
}

/// Represents a streaming type.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone)]
pub enum StreamType {
    /// Represents stream of events that are relevant to the authorized user, i.e. home timeline and notifications.
    User,
    /// Represents stream of all public statuses.
    Public,
    /// Represents stream of all local statuses.
    PublicLocal,
    /// Represents stream of all public statuses without local statuses. (mastodon v3.1.4 or later)
    PublicRemote,
    /// Represents stream of all public statuses for a particular hashtag.
    Hashtag(String),
    /// Represents stream of all local statuses for a particular hashtag.
    HashtagLocal(String),
    /// Represents stream of all statuses for a list.
    List(String),
    /// Represents stream of all direct messages.
    Direct,
}

use std::fmt;

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ENDPOINT: &str = "/api/v1/streaming";

        match self {
            StreamType::User => write!(f, "{}/user", ENDPOINT),
            StreamType::Public => write!(f, "{}/public", ENDPOINT),
            StreamType::PublicLocal => write!(f, "{}/public/local", ENDPOINT),
            StreamType::PublicRemote => write!(f, "{}/public/remote", ENDPOINT),
            StreamType::Hashtag(tag) => write!(f, "{}/hashtag?tag={}", ENDPOINT, tag),
            StreamType::HashtagLocal(tag) => write!(f, "{}/hashtag/local?tag={}", ENDPOINT, tag),
            StreamType::List(id) => write!(f, "{}/list?list={}", ENDPOINT, id),
            StreamType::Direct => write!(f, "{}/direct", ENDPOINT),
        }
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

