//! This module provides features related to notification.

pub mod id;

use std::collections::HashSet;
use serde::Serialize;
use crate::{
	Connection,
	Method,
	Result,
	entities::{ Notifications, NotificationType },
	private::MethodInternalWithoutRespHeader,
};

/// Get a request to get your received notifications.
pub fn get(conn: &Connection) -> GetNotifications {
	GetNotifications {
		conn,
		authorized: true,
		max_id: None,
		since_id: None,
		min_id: None,
		limit: None,
		exclude_types: None,
		account_id: None,
	}
}

/// GET request for `/api/v1/notifications`.
/// 
/// ## Note
/// Because Server response also contains `Link` HTTP response header for pagination but pagination is also possible with notification id,
/// This request not return `Link` HTTP response header.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Notifications, "/api/v1/notifications")]
pub struct GetNotifications<'a> {
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
	exclude_types: Option<Vec<String>>,
	account_id: Option<String>,
}

impl<'a> GetNotifications<'a> {
    /// Set to get notifications that have ID less than `max_id`.
	pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
		self.max_id = Some(max_id.into());
		self
	}

    /// Set to get latest notifications that have ID greater than `since_id`.
    /// 
    /// If an ID you specify is more than 20 older than the notification on the server, this method gets the latest 20 notifications in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest notification ID on the server
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

    /// Set to get notifications that have ID greater than `min_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest notification on the server, this method gets the oldest 20 notifications in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest notification ID on the server
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

	/// Set a limit number of notifications to get. Default is 20.
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
    }

	/// Set the `NotificationType`s to exclude.
	pub fn exclude_types(mut self, exclude_types: impl AsRef<[NotificationType]>) -> Self {
		let exclude_types = exclude_types.as_ref()
			.iter()
			.map(|n| n.to_owned())
			.collect::<HashSet<NotificationType>>();
		
		if exclude_types.is_empty() {
			self
		} else {
			self.exclude_types = Some(exclude_types
    			.iter()
    			.map(|n| n.to_string())
    			.collect::<Vec<String>>()
			);
			self
		}
	}

	/// Set an account ID that is origin of the notification.
	pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
		self.account_id = Some(account_id.into());
		self
	}
}

impl<'a> Method<'a, Notifications> for GetNotifications<'a> {
	fn send(&self) -> Result<Notifications> {
		use log::trace;
		use crate::private::{
			build_request,
		};
		use crate::utils;

		const QUERY_KEY: &str = "exclude_types[]";

		match &self.exclude_types {
			Some(et) => {
        		let req = build_request(self, reqwest::Method::GET)?.query(
        			&utils::build_array_query(QUERY_KEY, et)
				).build()?;
				trace!("Send a {} request to {}", req.method(), req.url());

				let res = self.conn.client().execute(req)?;
				trace!("{:?}", res);

				Ok(utils::check_response(res)?.json::<Notifications>()?)
			},
			None => {
				self.send_internal()
			}
		}

	}
}

/// This module provides features related to clear all of your received notifications from the server.
pub mod clear {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Nothing,
	};

	/// Get a request to clear all of your received notifications from the server.
	pub fn post(conn: &Connection) -> PostClearNotifications {
		PostClearNotifications {
			conn,
			authorized: true,
		}
	}

	/// POST request for `/api/v1/notifications/clear`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Nothing, "/api/v1/notifications/clear")]
	pub struct PostClearNotifications<'a> {
		#[serde(skip_serializing)]
		#[mastors(connection)]
		conn: &'a Connection,

		#[serde(skip_serializing)]
		#[mastors(authorization)]
		authorized: bool,
	}

	impl<'a> Method<'a, Nothing> for PostClearNotifications<'a> {}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_notifications() {
		let conn = Connection::new().unwrap();
		assert!(get(&conn).send().is_ok());
	}

	#[test]
	fn test_get_no_notifications_all_filterd() {
		let conn = Connection::new().unwrap();
		assert!(
			get(&conn).exclude_types([
				NotificationType::Follow,
				NotificationType::Favourite,
				NotificationType::Reblog,
				NotificationType::Mention,
				NotificationType::Poll,
				NotificationType::FollowRequest,
			])
			.send().unwrap().is_empty()
		);
	}
}
