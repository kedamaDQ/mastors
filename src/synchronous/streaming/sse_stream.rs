//! This module provides an implementation of [`StreamingTimeline`](./trait.StreamingTimeline.html) using Server-sent events.
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
use super::{
	EventType,
	StreamingTimeline,
};

/// Represents the stream of each timeline with Server-sent events.
pub struct SseStream {
    client: Client,
}

impl SseStream {
    pub(crate) fn new(url: Url, client: reqwest::blocking::Client) -> Self {
        SseStream {
            client: Client::new_with_client(url, client),
        }
    }
}

impl StreamingTimeline for SseStream {}

impl Iterator for SseStream {
    type Item = Result<EventType>;

    /// Get the next streaming event from streaming timeline.
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
            _ => {
				Ok(EventType::Unknown(
					format!("{}: {}", event_type.as_str(), event.data)
				))
			}
        }
    } else {
        Ok(EventType::Unknown(format!("{}: {}", "", event.data)))
    }
}
