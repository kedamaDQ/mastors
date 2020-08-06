//! This module provides features related to reblog.

/// This module provides features related to get accounts that reblogged the status.
pub mod reblogged_by {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Accounts,
	};

	/// Get a request to get accounts that reblogged a status specified by `id`.
	pub fn get(conn: &Connection, id: impl Into<String>) -> GetRebloggedBy {
		GetRebloggedBy {
			conn,
			id: id.into(),
			authorized: false,
		}
	}

	/// GET request for `/api/v1/statuses/:id/reblogged_by`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(GET, Accounts, "/api/v1/statuses/_PATH_PARAM_/reblogged_by")]
	pub struct GetRebloggedBy<'a> {
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

	impl<'a> Method<'a, Accounts> for GetRebloggedBy<'a> {}
}

/// This module provides features related to reblog the status.
pub mod reblog {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
		entities::Visibility,
	};

	/// Get a request to reblog a status specified by `id`.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostReblog {
		PostReblog {
			conn,
			id: id.into(),
			authorized: true,
			visibility: Visibility::Public,
		}
	}

	/// POST request for `/api/v1/statuses/:id/reblog`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/reblog")]
	pub struct PostReblog<'a> {
		#[serde(skip_serializing)]
		#[mastors(connection)]
		conn: &'a Connection,

		#[serde(skip_serializing)]
		#[mastors(path_param)]
		id: String,

		#[serde(skip_serializing)]
		#[mastors(authorization)]
		authorized: bool,

		visibility: Visibility,
	}

	impl<'a> PostReblog<'a> {
		/// Set visibility of reblog to public.
		/// This visibility parameter is not used as of Mastodon v3.1.4.
		pub fn public(mut self) -> Self {
			self.visibility = Visibility::Public;
			self
		}

		/// set visibility of reblog to unlisted.
		/// This visibility parameter is not used as of Mastodon v3.1.4.
		pub fn unlisted(mut self) -> Self {
			self.visibility = Visibility::Unlisted;
			self
		}

		/// set visibility of reblog to private.
		/// This visibility parameter is not used as of Mastodon v3.1.4.
		pub fn private(mut self) -> Self {
			self.visibility = Visibility::Private;
			self
		}
	}

	impl<'a> Method<'a, Status> for PostReblog<'a> {}
}

/// This module provides features related to unreblog the status.
pub mod unreblog {
	use serde::Serialize;
	use crate::{
		Connection,
		Method,
		entities::Status,
	};

	/// Get a request to unreblog the status specified by `id`.
	pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnreblog {
		PostUnreblog {
			conn,
			id: id.into(),
			authorized: true
		}
	}

	/// POST request for `/api/v1/statuses/:id/unreblog`.
	#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
	#[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/unreblog")]
	pub struct PostUnreblog<'a> {
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

	impl<'a> Method<'a, Status> for PostUnreblog<'a> {}
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
	fn test_reblog_unreblog() {
		let conn = Connection::new().unwrap();
		let posted = statuses::post(&conn, "reblog unreblog")
			.send()
			.unwrap()
			.status()
			.unwrap()
			.clone();
		let myself = posted.account();

		let reblogged = reblog::post(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.id(), reblogged.reblog().unwrap().id());

		let posted = statuses::id::get(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.reblogs_count(), 1);
		assert!(posted.reblogged());

		let got = reblogged_by::get(&conn, posted.id()).send().unwrap();
		assert_eq!(got.len(), 1);
		assert_eq!(got.get(0).unwrap().id(), myself.id());

		let unreblogged = unreblog::post(&conn, posted.id()).send().unwrap();
		assert_eq!(posted.id(), unreblogged.id());

		let got = reblogged_by::get(&conn, posted.id()).send().unwrap();
		assert!(got.is_empty());

		statuses::id::delete(&conn, posted.id()).send().unwrap();
	}
}
