//! This module provides features related to controlling single list specified by ID.
pub mod accounts;

use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::{ List, Nothing },
};

/// Get a request to get a list specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetList {
	GetList {
		conn,
		authorized: true,
		id: id.into(),
	}
}

/// Get a request to update title of the list specified by `id`.
pub fn put(conn: &Connection, id: impl Into<String>, title: impl Into<String>) -> PutList {
	PutList {
		conn,
		authorized: true,
		id: id.into(),
		title: title.into(),
	}
}

/// Get a request to delete a list specified by `id`.
pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteList {
	DeleteList {
		conn,
		authorized: true,
		id: id.into(),
	}
}

/// GET request for `/api/v1/lists/:id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, List, "/api/v1/lists/_PATH_PARAM_")]
pub struct GetList<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,
}

impl<'a> Method<'a, List> for GetList<'a> {}

/// PUT request for `/api/v1/lists/:id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(PUT, List, "/api/v1/lists/_PATH_PARAM_")]
pub struct PutList<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,

	title: String,
}

impl<'a> Method<'a, List> for PutList<'a> {}

/// DELETE request for `/api/v1/lists/:id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(DELETE, Nothing, "/api/v1/lists/_PATH_PARAM_")]
pub struct DeleteList<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,
}

impl<'a> Method<'a, Nothing> for DeleteList<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::v1::lists;

	#[test]
	fn test_post_get_put_delete_list() {
		let conn = Connection::new().unwrap();
		let title = "mastorstestlistbeforeupdate";

		let posted = lists::post(&conn, title).send().unwrap();
		assert_eq!(posted.title(), title);

		let got = get(&conn, posted.id()).send().unwrap();
		assert_eq!(got.title(), title);

		let title_updated = "mastorstestlistafterupdate";
		let put = put(&conn, posted.id(), title_updated).send().unwrap();
		assert_eq!(put.title(), title_updated);

		let got = get(&conn, posted.id()).send().unwrap();
		assert_eq!(got.title(), title_updated);

		assert!(
			delete(&conn, got.id()).send().is_ok()
		);
	}
}
