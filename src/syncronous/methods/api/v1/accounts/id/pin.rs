//! This module provides features related to pin the account on your public profile.
//! Pinned accounts is also called featured accounts.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Relationship,
};

/// Get a request to pin an account specified by `id`.
pub fn post(conn: &Connection, id: impl Into<String>) -> PostPin {
	PostPin {
		conn,
		id: id.into(),
		authorized: true,
	}
}

/// POST request for `/api/v1/accounts/:id/pin`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/pin")]
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

impl<'a> Method<'a, Relationship> for PostPin<'a> {}

#[cfg(test)]
mod tests {
	// fmm...
}
