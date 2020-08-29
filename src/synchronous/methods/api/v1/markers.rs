//! This module provides features related to track unread statuses or unchecked notifications across sessions. 
use serde::Serialize;
use crate::{
	Connection,
	Error,
	Method,
	Result,
	entities::Markers,
};

/// Get a request to get markers that represents the position you read on the timeline.
pub fn get(conn: &Connection) -> GetMarkers {
	GetMarkers {
		conn,
		authorized: true,
		timelines: Timelines::new(),
	}
}

/// Get a request to set markers that represents the position you read on the timeline.
pub fn post(conn: &Connection) -> PostMarkers {
	PostMarkers {
		conn,
		authorized: true,
		home: None,
		notifications: None,
	}
}

/// GET request for `/api/v1/markers`.
/// 
/// This request get markers of both of home timeline and notifications timeline by default.
/// If you really want to get only one of them, use [`without_home()`](#method.without_home) or [`without_notifications()`](#method.without_notifications).
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Markers, "/api/v1/markers")]
pub struct GetMarkers<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
	timelines: Timelines,
}

impl<'a> GetMarkers<'a> {
	/// Set to exclude the marker of home timeline from response.
	pub fn without_home(mut self) -> Self {
		self.timelines.inner.remove(&Timeline::Home);
		self
	}

	/// Set to exclude the marker of notifications timeline from response.
	pub fn without_notifications(mut self) -> Self {
		self.timelines.inner.remove(&Timeline::Notifications);
		self
	}
}

impl<'a> Method<'a, Markers> for GetMarkers<'a> {
	/// This method will return error if:
	/// 
	/// - No timeline specified
	fn send(&self) -> Result<Markers> {
		use log::trace;
		use crate::private::{
			build_request,
		};
		use crate::utils;

		const QUERY_KEY: &str = "timeline[]";

		self.timelines.validate()?;

		let req = build_request(self, reqwest::Method::GET)?.query(
			&utils::build_array_query(
				QUERY_KEY,
				self.timelines.inner
					.iter()
					.map(|tl| tl.to_string())
					.collect::<Vec<String>>()
					.as_slice()
			)
		).query(self).build()?;
		trace!("Send a {} request to {}", req.method(), req.url());

		let res = self.conn.client().execute(req)?;
		trace!("{:?}", res);

		Ok(utils::check_response(res)?.json::<Markers>()?)
	}
}

/// POST request for `/api/v1/markers`.
/// 
/// This request requires at least either status ID on the home timeline or notification ID on the notifications timeline.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Markers, "/api/v1/markers")]
pub struct PostMarkers<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	home: Option<Vec<String>>,
	notifications: Option<Vec<String>>,
}

impl<'a> PostMarkers<'a> {
	/// Set an `id` of status you have read.
	pub fn home(mut self, id: impl Into<String>) -> Self {
		self.home = Some(vec![id.into()]);
		self
	}

	/// Set an `id` of notification you have checked.
	pub fn notifications(mut self, id: impl Into<String>) -> Self {
		self.notifications = Some(vec![id.into()]);
		self
	}

	fn validate(&self) -> Result<()> {
		if self.home.is_none() && self.notifications.is_none() {
			return Err(
				Error::NoTimelineError
			);
		}
		Ok(())
	}
}

impl<'a> Method<'a, Markers> for PostMarkers<'a> {
	fn send(&self) -> Result<Markers> {
		use crate::private::MethodInternalWithoutRespHeader;

		self.validate()?;
		self.send_internal()
	}
}

/// Represents timelines that you will get or post.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
enum Timeline {
	/// Represents the home timeline.
	Home,

	/// Represents the notifications timeline.
	Notifications,
}

use std::fmt;

impl fmt::Display for Timeline {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Timeline::Home => write!(f, "home"),
			Timeline::Notifications => write!(f, "notifications"),
		}
	}
}

use serde::ser;

impl ser::Serialize for Timeline {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: ser::Serializer
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}

use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
struct Timelines {
	inner: HashSet<Timeline>,
}

impl Timelines {
	fn new() -> Self {
		let mut inner = HashSet::new();
		inner.insert(Timeline::Home);
		inner.insert(Timeline::Notifications);

		Timelines {
			inner
		}
	}

	fn validate(&self) -> Result<()> {
		if self.inner.is_empty() {
			return Err(
				Error::NoTimelineError
			);
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::v1::timelines;
	use crate::api::v1::notifications;

	#[test]
	fn test_post_get_markers() {
		let conn = Connection::new().unwrap();

		let home_id = timelines::home::get(&conn)
			.send().unwrap()
			.get(0).unwrap()
			.id().to_owned();
		let notification_id = notifications::get(&conn)
			.send().unwrap()
			.get(0).unwrap()
			.id().to_owned();

		let posted = post(&conn)
			.home(home_id)
			.notifications(notification_id)
			.send().unwrap();


		let got = get(&conn).send().unwrap();

		assert_eq!(
			posted.home().unwrap().last_read_id(),
			got.home().unwrap().last_read_id()
		);

		assert_eq!(
			posted.notifications().unwrap().last_read_id(),
			got.notifications().unwrap().last_read_id()
		);
	}

	#[test]
	fn test_get_single_marker() {
		let conn = Connection::new().unwrap();

		let home_id = timelines::home::get(&conn)
			.send().unwrap()
			.get(0).unwrap()
			.id().to_owned();
		let notification_id = notifications::get(&conn)
			.send().unwrap()
			.get(0).unwrap()
			.id().to_owned();

		let posted = post(&conn)
			.home(home_id)
			.notifications(notification_id)
			.send().unwrap();

		let got = get(&conn)
			.without_home()
			.send().unwrap();

		assert!(got.home().is_none());
		assert_eq!(
			got.notifications().unwrap().last_read_id(),
			posted.notifications().unwrap().last_read_id()
		);
	}

	#[test]
	fn test_get_no_timeline_error() {
		let conn = Connection::new().unwrap();
		assert!(get(&conn)
			.without_home()
			.without_notifications()
			.send()
			.is_err()
		);
	}

	#[test]
	fn test_post_no_timeline_error() {
		let conn = Connection::new().unwrap();
		assert!(post(&conn).send().is_err());
	}
}
