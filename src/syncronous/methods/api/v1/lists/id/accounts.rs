//! This module provides features related to member accounts of list specified by ID.
use serde::Serialize;
use crate::{
	Connection,
	Error,
	Method,
	MethodWithRespHeader,
	Result,
	methods::MethodInternalWithoutRespHeader,
	entities::{ Accounts, Nothing },
};

/// Get a request to get member accounts of list specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetListAccounts {
	GetListAccounts {
		conn,
		id: id.into(),
		authorized: true,
		max_id: None,
		since_id: None,
		limit: None,
	}
}

/// Get request to add accounts to list specified by `id`.
pub fn post<T, U>(
	conn: &Connection,
	id: impl Into<String>,
	account_ids: T
) -> PostListAccounts
where
	T: AsRef<[U]>,
	U: AsRef<str>,
{
	let account_ids = account_ids.as_ref()
		.iter()
		.map(|i| i.as_ref().trim())
		.filter(|i| ! i.is_empty())
		.map(|i| i.to_owned())
		.collect::<Vec<String>>();

	PostListAccounts {
		conn,
		id: id.into(),
		authorized: true,
		account_ids,
	}
}

/// Get request to remove accounts from list specified by `id`.
pub fn delete<T, U>(
	conn: &Connection,
	id: impl Into<String>,
	account_ids: T
) -> DeleteListAccounts
where
	T: AsRef<[U]>,
	U: AsRef<str>,
{
	let account_ids = account_ids.as_ref()
		.iter()
		.map(|i| i.as_ref().trim())
		.filter(|i| ! i.is_empty())
		.map(|i| i.to_owned())
		.collect::<Vec<String>>();

	DeleteListAccounts {
		conn,
		id: id.into(),
		authorized: true,
		account_ids,
	}
}

/// GET request for `/api/v1/lists/:id/accounts`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Accounts, "/api/v1/lists/_PATH_PARAM_/accounts", "Link")]
pub struct GetListAccounts<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	max_id: Option<String>,
	since_id: Option<String>,
	limit: Option<usize>,
}

impl<'a> GetListAccounts<'a> {
	/// This option is a pagination parameter.
	/// Set the account ID that is last of account list you are showing to get next page.
	pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
		self.max_id = Some(max_id.into());
		self
	}

	/// This option is a pagination parameter.
	/// Set the account ID that is first of account list you are showing to get previous page.
	pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
		self.since_id = Some(since_id.into());
		self
	}

	/// Set max number of accounts. default is 40 and also max is 40.
	/// If set 0 then return all of member accounts of list.
	//  Specified by /app/controllers/api/base_controller.rb#DEFAULT_ACCOUNTS_LIMIT,
	//  /app/controllers/api/v1/lists/accounts_controller.rb#unlimited?
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
	}
}

impl<'a> MethodWithRespHeader<'a, Accounts> for GetListAccounts<'a> {}

/// POST request for `/api/v1/lists/:id/accounts`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Nothing, "/api/v1/lists/_PATH_PARAM_/accounts")]
pub struct PostListAccounts<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	account_ids: Vec<String>,
}

impl<'a> Method<'a, Nothing> for PostListAccounts<'a> {
	/// This method will return error if:
	/// - `account_id` is empty or contains only whitespace or blank
	/// - `account_id` contains duplicate ids
	fn send(&'a self) -> Result<Nothing> {
		use std::collections::HashSet;

		if self.account_ids.is_empty() {
			return Err(
				Error::NoAccountIdError
			);
		}

		if self.account_ids.iter().collect::<HashSet<&String>>().len() != self.account_ids.len() {
			return Err(
				Error::DuplicateAccountIdError
			)
		}

		self.send_internal()
	}
}

/// DELETE request for `/api/v1/lists/:id/accounts`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(DELETE, Nothing, "/api/v1/lists/_PATH_PARAM_/accounts")]
pub struct DeleteListAccounts<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(path_param)]
	id: String,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	account_ids: Vec<String>,
}

impl<'a> Method<'a, Nothing> for DeleteListAccounts<'a> {
	/// This method will return error if:
	/// - `account_id` is empty or contains only whitespace or blank
	/// - `account_id` contains duplicate ids
	fn send(&'a self) -> Result<Nothing> {
		use std::collections::HashSet;

		if self.account_ids.is_empty() {
			return Err(
				Error::NoAccountIdError
			);
		}

		if self.account_ids.iter().collect::<HashSet<&String>>().len() != self.account_ids.len() {
			return Err(
				Error::DuplicateAccountIdError
			)
		}

		self.send_internal()
	}
}

#[cfg(test)]
mod tests {
	// fmm...if test this module, need a long scenario.
}
