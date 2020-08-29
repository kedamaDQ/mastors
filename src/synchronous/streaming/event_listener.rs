//! This module provides event listener that to use streaming timeline conveniently.
//! 
//! ## Note
//! 
//! This module is experimental.
use std::result::Result as StdResult;
use std::error::Error as StdError;
use crate::{
	entities::{
		Notification,
		Status,
	},
};

/// This trait provides feature for handling streaming events.
/// 
/// Override any methods of `EventListener` you need, for example,
/// [`update()`](#method.update),
/// [`notification()`](`#method.notification`)
/// in the implementation of `EventListener` and pass to [`StreamingTimeline.attach()`](./trait.StreamingTimeline.html#method.attach).
/// 
pub trait EventListener {
	/// Error type that will return from overrided methods.
	type Error: Into<Box<dyn StdError>>;

	/// This method will called when receive `update` event from streaming timeline.
	#[allow(unused_variables)]
	fn update(&self, status: &Status) -> StdResult<(), Self::Error> {
		Ok(())
	}

	/// This method will called when receive `notification` event from the user streaming timeline.
	#[allow(unused_variables)]
	fn notification(&self, notification: &Notification) -> StdResult<(), Self::Error> {
		Ok(())
	}

	/// This method will called when receive `update` event from streaming timeline.
	#[allow(unused_variables)]
	fn delete(&self, deleted_status: impl AsRef<str>) -> StdResult<(), Self::Error> {
		Ok(())
	}

	/// This method will called when receive `filters_chenged` event from streaming timeline.
	fn filters_changed(&self) -> StdResult<(), Self::Error> {
		Ok(())
	}

	/// This method will called when receive unknown event from streaming timeline.
	/// 
	/// EventListener determines that an event has no type or has unknown type is `unkown`.
	#[allow(unused_variables)]
	fn unknown(&self, msg: impl AsRef<str>) -> StdResult<(), Self::Error> {
		Ok(())
	}
}
