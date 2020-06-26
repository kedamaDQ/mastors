//! This module provides features related to unpin the account on your public profile.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Relationship,
};

/// Get a request to unpin an account specified by `id`.
pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnpin {
	PostUnpin {
		conn,
		id: id.into(),
		authorized: true,
	}
}

/// POST request for `/api/v1/accounts/:id/unpin`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/unpin")]
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

impl<'a> Method<'a, Relationship> for PostUnpin<'a> {}

#[cfg(test)]
mod tests {
	// fmm...
}
