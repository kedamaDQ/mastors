//! This module provides the streaming timeline and the parts to process streamed items.
mod event_listener;
mod event_type;
mod sse_stream;
mod stream_type;

pub use event_listener::EventListener;
pub use event_type::EventType;
pub use sse_stream::SseStream;
pub use stream_type::StreamType;

use std::result::Result as StdResult;
use std::error::Error as StdError;
use log::{ debug, trace };
use crate::{
	Result,
};

/// This trait represents the abstract streaming timeline.
/// 
/// Streaming timeline implements an `Iterator` it means you can use in loop and process events sequentially.
/// Also you can pass the [`EventListener`](./trait.EventListener.html) to `StreamingTimeline` with [`attach()`](#method.attach) to process events.
pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {
    /// Attach an implementation of [`EventListener`](./trait.EventListener.html) to this streaming timeline.
	fn attach(&mut self, listener: &impl EventListener) -> StdResult<(), Box<dyn StdError>> {
        debug!("Attach to streaming timeline");

		for event in self.into_iter() {
            match event {
                Ok(event_type) => {
                    if let Err(e) = dispatch_event(listener, event_type) {
                        return Err(e.into());
                    }
                },
                Err(e) => {
                    return Err(e.into());
                }
            };
		}
		Ok(())
	}
}

fn dispatch_event<E>(
    listener: &impl EventListener<Error = E>,
    event_type: EventType
) -> StdResult<(), E>
where
    E: Into<Box<dyn StdError>>,
{
    match event_type {
        EventType::Update(status) => {
            trace!("Dispatch an update to listener");
            listener.update(status.as_ref())
        },
        EventType::Notification(notification) => {
            trace!("Dispatch a notification to listener");
            listener.notification(notification.as_ref())
        },
        EventType::Delete(status_id) => {
            trace!("Dispatch a delete to listener");
            listener.delete(status_id)
        },
        EventType::FiltersChanged => {
            trace!("Dispatch a filters_changed to listener");
            listener.filters_changed()
        },
        EventType::Unknown(msg) => {
            trace!("Dispatch an unknown message to listener: {}", msg);
            listener.unknown(msg)
        },
    }
}
