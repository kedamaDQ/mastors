//! This module provides features related to muting conversations.

/// This module provides features related to mute notifications from the conversation.
pub mod mute {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
	};

	/// Get a request to mute notification from the conversation that contains status specified by `id`.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostMute {
		PostMute {
			conn,
			id: id.into(),
			authorized: true,
		}
	}

	/// POST request for `/api/v1/statuses/:id/mute`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/mute")]
	pub struct PostMute<'a> {
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

	impl<'a> Method<'a, Status> for PostMute<'a> {}
}

/// This module provides features related to unmute notification from the conversation.
pub mod unmute {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
	};

	/// Get a request to unmute notification from the conversation that contains status specified by `id`.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnmute {
		PostUnmute {
			conn,
			id: id.into(),
			authorized: true,
		}
	}

	/// POST request for `/api/v1/statuses/:id/unmute`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/unmute")]
	pub struct PostUnmute<'a> {
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

	impl<'a> Method<'a, Status> for PostUnmute<'a> {}
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
	fn test_mute_unmute() {
		let conn = Connection::new().unwrap();
		let posted = statuses::post(&conn, "mute unmute")
			.send()
			.unwrap()
			.status()
			.unwrap()
			.clone();
		let replied = statuses::post(&conn, "mute unmute reply")
			.in_reply_to_id(posted.id())
			.send()
			.unwrap()
			.status()
			.unwrap()
			.clone();

		let muted = mute::post(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.id(), muted.id());

		let got = statuses::id::get(&conn, muted.id()).authorized().send().unwrap();
		assert!(got.muted());

		let got2 = statuses::id::get(&conn, replied.id()).authorized().send().unwrap();
		assert!(got2.muted());

		let unmuted = unmute::post(&conn, muted.id()).send().unwrap();
		assert_eq!(unmuted.id(), muted.id());

		let got = statuses::id::get(&conn, muted.id()).authorized().send().unwrap();
		assert!(! got.muted());

		let got2 = statuses::id::get(&conn, replied.id()).authorized().send().unwrap();
		assert!(! got2.muted());

		statuses::id::delete(&conn, posted.id()).send().unwrap();
		statuses::id::delete(&conn, replied.id()).send().unwrap();
	}
}
