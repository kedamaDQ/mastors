//! This module provides features related to search accounts.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Accounts,
};

/// Get a request to search accounts with keyword `q`.
pub fn get(conn: &Connection, q: impl Into<String>) -> GetSearch {
	GetSearch {
		conn,
		authorized: true,
		q: q.into(),
		limit: None,
		resolve: None,
		following: None,
	}
}

/// GET request for `/api/v1/accounts/search`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Accounts, "/api/v1/accounts/search")]
pub struct GetSearch<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	// Required params
	q: String,

	// Optional params
	limit: Option<usize>,
	resolve: Option<bool>,
	following: Option<bool>,
}

impl<'a> GetSearch<'a> {
	/// Set a limit on the number of search results.
	/// Defaults to 40.
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
	}

	/// Set to attempt WebFinger lookup.
	/// Use this when search keyword is an exact address.
	pub fn resolve(mut self) -> Self {
		self.resolve = Some(true);
		self
	}

	/// Set search target to accounts you are following only. 
	pub fn following(mut self) -> Self {
		self.following = Some(true);
		self
	}
}

impl<'a> Method<'a, Accounts> for GetSearch<'a> {}
