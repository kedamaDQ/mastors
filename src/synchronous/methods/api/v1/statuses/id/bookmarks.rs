//! This module provides features related to controlling status bookmarks.

pub mod bookmark {
	/// This module provides features related to bookmark a status.

    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Status,
    };
    
    /// Get a request to bookmark a status specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostBookmark {
    	PostBookmark {
    		conn,
    		id: id.into(),
    		authorized: true,
    	}
    }
    
    /// POST request for `/api/v1/statuses/:id/bookmark`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/bookmark")]
    pub struct PostBookmark<'a> {
    	#[serde(skip_serializing)]
    	#[mastors(connection)]
    	conn: &'a Connection,
    
    	#[serde(skip_serializing)]
    	#[mastors(path_param)]
    	id: String,
    
    	#[serde(skip_serializing)]
    	#[mastors(authorization)]
    	authorized: bool
    }
    
    impl<'a> Method<'a, Status> for PostBookmark<'a> {}
}

pub mod unbookmark {
    //! This module provides features related to unbookmark a status.
    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Status,
    };
    
    /// Get a request to unbookmark a status specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnbookmark {
    	PostUnbookmark {
    		conn,
    		id: id.into(),
    		authorized: true,
    	}
    }
    
    /// POST request for `/api/v1/statuses/:id/bookmark`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/unbookmark")]
    pub struct PostUnbookmark<'a> {
    	#[serde(skip_serializing)]
    	#[mastors(connection)]
    	conn: &'a Connection,
    
    	#[serde(skip_serializing)]
    	#[mastors(path_param)]
    	id: String,
    
    	#[serde(skip_serializing)]
    	#[mastors(authorization)]
    	authorized: bool
    }
    
    impl<'a> Method<'a, Status> for PostUnbookmark<'a> {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Method;

	#[test]
	fn test_bookmark_and_unbookmark() {
		let conn = crate::Connection::new().unwrap();

		let status = crate::api::v1::statuses::post(&conn)
			.status("bookmark and unbookmark.")
			.send()
			.unwrap();

		let bookmarked = bookmark::post(&conn, status.id()).send().unwrap();
		assert_eq!(status.id(), bookmarked.id());
		assert!(bookmarked.bookmarked());

		let unbookmarked = unbookmark::post(&conn, status.id()).send().unwrap();
		assert_eq!(status.id(), unbookmarked.id());
		assert!(! unbookmarked.bookmarked());

		crate::api::v1::statuses::id::delete(&conn, status.id()).send().unwrap();
	}
}
