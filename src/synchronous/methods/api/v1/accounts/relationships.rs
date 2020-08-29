//! This module provides features related to your relationships that are followed, following, blocking, muting, etc...
//! 
//! ## Note
//! This module requires importing `Method` to use.
//! ```rust
//! use mastors::Method;
//! ```
use serde::Serialize;
use crate::{
	Connection,
	Error,
	Method,
	Result,
	entities::Relationships,
};

/// Get a request to get your relationships specified by `ids`.
pub fn get<T, U>(conn: &Connection, ids: T) -> GetRelationships
where
	T: AsRef<[U]>,
	U: AsRef<str>
{
	GetRelationships {
		conn,
		authorized: true,
		id: RelationshipIds::new(ids),
	}
}

/// GET request for `/api/v1/accounts/relationships`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Relationships, "/api/v1/accounts/relationships")]
pub struct GetRelationships<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
	id: RelationshipIds,
}

impl<'a> Method<'a, Relationships> for GetRelationships<'a> {
	fn send(&self) -> Result<Relationships> {
		use log::trace;
		use crate::private::{
			build_request,
		};
		use crate::utils;

		const QUERY_KEY: &str = "id[]";

		self.id.validate()?;

		let req = build_request(self, reqwest::Method::GET)?.query(
			utils::build_array_query(QUERY_KEY, &self.id.inner).as_slice()
		).build()?;
		trace!("Send a {} request to {}", req.method(), req.url());

		let res = self.conn.client().execute(req)?;
		trace!("{:?}", res);

		Ok(utils::check_response(res)?.json::<Relationships>()?)
	}
}

#[derive(Debug, Clone)]
struct RelationshipIds {
	inner: Vec<String>,
}

impl RelationshipIds {
	fn new<T, U>(ids: T) -> Self
	where
		T: AsRef<[U]>,
		U: AsRef<str>,
	{
		let ids = ids.as_ref()
			.iter()
			.map(|u| u.as_ref().trim())
			.filter(|u| !u.is_empty())
			.map(|u| u.to_owned())
			.collect::<Vec<String>>();
		
		RelationshipIds {
			inner: ids,
		}
	}

	fn validate(&self) -> Result<()> {
		use std::collections::HashSet;

		if self.inner.is_empty() {
			return Err(Error::NoAccountIdError)
		}

		if self.inner.iter().collect::<HashSet<&String>>().len() != self.inner.len() {
			return Err(Error::DuplicateAccountIdError)
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_relationships_validation_empty() {
		let conn = Connection::new().unwrap();
		assert!(get(&conn, [" ", " "]).send().is_err());
	}

	#[test]
	fn test_get_relationships_validation_duplicate() {
		let conn = Connection::new().unwrap();
		assert!(get(&conn, ["2", "2"]).send().is_err());
	}
}
