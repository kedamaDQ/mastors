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
use crate::{
	Result,
};

/// This trait represents the abstract streaming timeline.
/// 
/// Streaming timeline implements an `Iterator` it means you can use in loop and process events sequentially.
/// Also you can pass the [`EventListener`](./trait.EventListener.html) to `StreamingTimeline` with [`attach()`](#method.attach) to process events.
pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {
    /// To attach an implementation of [`EventListener`](./trait.EventListener.html).
	fn attach(&mut self, listener: impl EventListener) -> StdResult<(), Box<dyn StdError>> {
		for event in self.into_iter() {
            match event {
                Ok(event_type) => {
                    if let Err(e) = dispatch_event(&listener, event_type) {
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
            listener.update(status.as_ref())
        },
        EventType::Notification(notification) => {
            listener.notification(notification.as_ref())
        },
        EventType::Delete(status_id) => {
            listener.delete(status_id)
        },
        EventType::FiltersChanged => {
            listener.filters_changed()
        },
        EventType::Unknown(msg) => {
            listener.unknown(msg)
        },
    }
}
