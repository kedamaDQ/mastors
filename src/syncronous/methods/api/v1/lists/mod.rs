//! This module provides features related to controlling list timeline.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::{ List, Lists, Nothing },
};

/// Get a request to get all your lists.
pub fn get(conn: &Connection) -> GetLists {
	GetLists {
		conn,
		authorized: true,
	}
}

/// Get a request to create a new list timeline named by `title`.
pub fn post(conn: &Connection, title: impl Into<String>) -> PostLists {
	PostLists {
		conn,
		authorized: true,
		title: title.into(),
	}
}

/// Get a request to delete a list specified by `id`.
pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteLists {
	DeleteLists {
		conn,
		authorized: true,
		id: id.into(),
	}
}

/// GET request for `/api/v1/lists`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Lists, "/api/v1/lists")]
pub struct GetLists<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,
}

impl<'a> Method<'a, Lists> for GetLists<'a> {}

/// POST request for `/api/v1/lists`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, List, "/api/v1/lists")]
pub struct PostLists<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	title: String,
}

impl<'a> Method<'a, List> for PostLists<'a> {}

/// DELETE request for `/api/v1/lists`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(DELETE, Nothing, "/api/v1/lists/_PATH_PARAM_")]
pub struct DeleteLists<'a> {
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

impl<'a> Method<'a, Nothing> for DeleteLists<'a> {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_post_delete_lists() {
		let conn = Connection::new().unwrap();
		let title = "mastorstestlist";

		let posted = post(&conn, title).send().unwrap();
		assert_eq!(posted.title(), title);

		let got = get(&conn).send().unwrap();
		let expect_posted = got.iter()
			.filter(|l| l.id() == posted.id())
			.collect::<Vec<&List>>();
		assert!(expect_posted.len() == 1);
		assert_eq!(expect_posted.get(0).unwrap().title(), posted.title());
		
		println!("{:?}", delete(&conn, posted.id()).send());
	}
}
