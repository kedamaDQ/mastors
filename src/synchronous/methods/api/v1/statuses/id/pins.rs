//! This module provides features related to pinning status.

/// This module provides features related to pin the status to your public profile.
pub mod pin {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
	};

	/// Get a request to pin the status specified by `id` to your public profile.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostPin {
		PostPin {
			conn,
			id: id.into(),
			authorized: true,
		}
	}

	/// POST request for `/api/v1/statuses/:id/pin`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/pin")]
	pub struct PostPin<'a> {
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

	impl<'a> Method<'a, Status> for PostPin<'a> {}
}

/// This module provides features related to unpin the status from your public profile.
pub mod unpin {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
	};

	/// Get a request to uppin the status specified by `id` from your public profile.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnpin {
		PostUnpin {
			conn,
			id: id.into(),
			authorized: true,
		}
	}

	/// POST request for `/api/v1/statuses/:id/unpin`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/unpin")]
	pub struct PostUnpin<'a> {
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

	impl<'a> Method<'a, Status> for PostUnpin<'a> {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Connection,
		Method,
		api::v1::statuses,
	};

	#[test]
	fn test_pin_unpin() {
		let conn = Connection::new().unwrap();
		let posted = statuses::post(&conn, "pin unpin")
			.send()
			.unwrap()
			.status()
			.unwrap()
			.clone();

		let pinned = pin::post(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.id(), pinned.id());

		let got = statuses::id::get(&conn, posted.id()).authorized().send().unwrap();
		assert_eq!(posted.id(), got.id());
		assert!(got.pinned());

		let unpinned = unpin::post(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.id(), unpinned.id());

		let got = statuses::id::get(&conn, posted.id()).authorized().send().unwrap();
		assert_eq!(posted.id(), got.id());
		assert!(! got.pinned());

		assert!(statuses::id::delete(&conn, posted.id()).send().is_ok());
	}
}
