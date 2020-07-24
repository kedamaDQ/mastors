use serde::Deserialize;
use crate::{ DateTime, Utc };

/// Represents the last read ID of the status and notification.
/// 
/// Any element to be None if call for the api method without specified elements.
/// See [`/api/v1/markers`](../syncronous/methods/api/v1/markers).
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Markers {
	home: Option<Marker>,
	notifications: Option<Marker>,
}

impl Markers {
	/// Get a last read ID of status on the home timeline.
	pub fn home(&self) -> Option<&Marker> {
		self.home.as_ref()
	}

	/// Get a last read ID of notification on the notifications timeline.
	pub fn notifications(&self) -> Option<&Marker> {
		self.notifications.as_ref()
	}
}

/// Represents the last read ID of the status or notification.
#[derive(Debug, Clone, Deserialize)]
pub struct Marker {
	last_read_id: String,
	updated_at: DateTime<Utc>,
	version: u32,
}

impl Marker {
	/// Get an ID of status or notification you read.
	pub fn last_read_id(&self) -> &str {
		&self.last_read_id
	}

	/// Get updated date and time of this marker.
	pub fn updated_at(&self) -> DateTime<Utc> {
		self.updated_at
	}

	/// Get a version number of this marker.
	/// 
	/// This number incremented at every update to prevent update conflict.
	pub fn version(&self) -> u32 {
		self.version
	}
}
