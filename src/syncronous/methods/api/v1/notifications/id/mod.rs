//! This module provides features related to controlling single notification specifyed by ID.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Notification,
};

/// Get a request to get notification specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetNotification {
	GetNotification {
		conn,
		id: id.into(),
		authorized: true,
	}
}

/// GET request for `/api/v1/notifications/:id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Notification, "/api/v1/notifications/_PATH_PARAM_")]
pub struct GetNotification<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,
}

impl<'a> Method<'a, Notification> for GetNotification<'a> {}

/// This module provides features related to dismiss single notification specified by ID.
pub mod dismiss {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Nothing,
	};

	/// Get a request to dismiss a notification specified by `id`.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostDismissNotification {
		PostDismissNotification {
			conn,
			id: id.into(),
			authorized: true
		}
	}

	/// POST requets for `/api/v1/notifications/:id/dismiss`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Nothing, "/api/v1/notifications/_PATH_PARAM_/dismiss")]
	pub struct PostDismissNotification<'a> {
		#[serde(skip_serializing)]
		#[mastors(connection)]
		conn: &'a Connection,

		#[serde(skip_serializing)]
		#[mastors(path_param)]
		id: String,

		#[serde(skip_serializing)]
		#[mastors(authorization)]
		authorized: bool,
	}

	impl<'a> Method<'a, Nothing> for PostDismissNotification<'a> {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::v1::notifications;

	#[test]
	fn test_get_notification() {
		let conn = Connection::new().unwrap();
		let notifications = notifications::get(&conn).send().unwrap();

		assert!(
			get(&conn, notifications.get(0).unwrap().id()).send().is_ok()
		);
	}
}
