//! This module provides features related to streaming timelines.
mod event_handler;
mod event_type;
mod sse_stream;
mod stream_type;

pub use event_handler::EventHandler;
pub use event_type::EventType;
pub use sse_stream::SseStream;
pub use stream_type::StreamType;

use crate::{
	Result,
};

pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {

	fn attach(&mut self, handler: impl EventHandler) -> std::result::Result<(), Box<dyn std::error::Error>> {
		for event in self.into_iter() {
            match event {
                Ok(event_type) => {
                    if let Err(e) = handler.handle(event_type) {
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
