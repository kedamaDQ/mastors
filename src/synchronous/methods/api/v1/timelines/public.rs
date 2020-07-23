//! This module provides features related to get the public timeline.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Statuses,
};

/// Get a request to get statuses of public timeline.
pub fn get(conn: &Connection) -> GetPublicTimeline {
	GetPublicTimeline {
		conn,
		authorized: conn.public_timeline_preview_disabled(),
		max_id: None,
		since_id: None,
		min_id: None,
		limit: None,
		local: None,
		remote: None,
		only_media: None,
	}
}

/// GET request for `/api/v1/timelines/public`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Statuses, "/api/v1/timelines/public")]
pub struct GetPublicTimeline<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	max_id: Option<String>,
	since_id: Option<String>,
	min_id: Option<String>,
	limit: Option<usize>,

	local: Option<bool>,
	remote: Option<bool>,
	only_media: Option<bool>,
}

impl<'a> GetPublicTimeline<'a> {
	/// Set the Authorization header to this GET request.
	/// Authorization header is force ON if `PUBLIC_TIMELINE_PREVIEW_DISABLED` is set in connection settings.
	pub fn authorized(mut self) -> Self {
		self.authorized = true;
		self
	}

    /// Set to get statuses that have ID less than `max_id`.
	pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
		self.max_id = Some(max_id.into());
		self
	}

    /// Set to get latest statuses that have ID greater than `since_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the latest 20 statuses in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃
    /// ┃┃ min_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ┣ ID you specified on the server
    /// ：
    /// ：
    /// ```
	pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
		self.since_id = Some(since_id.into());
		self
	}

    /// Set to get statuses that have ID greater than `min_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the oldest 20 statuses in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃
    /// ┃┃ min_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ┣ ID you specified on the server
    /// ：
    /// ：
    /// ```
	pub fn min_id(mut self, min_id: impl Into<String>) -> Self {
		self.min_id = Some(min_id.into());
		self
	}

	/// Set a limit number of statuses to get.
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
	}

	/// Set to get only statuses that are posted from the connected server.
	pub fn local(mut self) -> Self {
		self.local = Some(true);
		self
	}

	/// Set to get only statuses that are posted from the server other than the connected server.
	pub fn remote(mut self) -> Self {
		self.remote = Some(true);
		self
	}

	/// Set to get only statuses that are media attached.
	pub fn only_media(mut self) -> Self {
		self.only_media = Some(true);
		self
	}
}

impl<'a> Method<'a, Statuses> for GetPublicTimeline<'a> {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_public_timeline() {
		let conn = Connection::new().unwrap();

		assert!(
			get(&conn).authorized().send().is_ok()
		);
	}
}
