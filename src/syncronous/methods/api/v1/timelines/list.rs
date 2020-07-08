//! This module provides features related to get the list timeline.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Statuses,
};

/// Get a request to get the list timeline specified by `list_id`.
pub fn get(conn: &Connection, list_id: impl Into<String>) -> GetListTimeline {
    GetListTimeline {
        conn,
        authorized: true,
        list_id: list_id.into(),
        max_id: None,
        since_id: None,
        min_id: None,
        limit: None,
    }
}

/// GET request for `/api/v1/timelines/list/:list_id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Statuses, "/api/v1/timelines/list/_PATH_PARAM_")]
pub struct GetListTimeline<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

	#[serde(skip_serializing)]
    #[mastors(path_param)]
    list_id: String,

	max_id: Option<String>,
	since_id: Option<String>,
	min_id: Option<String>,
	limit: Option<usize>,
}

impl<'a> GetListTimeline<'a> {
    /// Set to get statuses that have ID less than `max_id`.
	pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
		self.max_id = Some(max_id.into());
		self
	}

    /// Set to get latest statuses that have ID greater than `since_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the latest 20 statuses in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃
    /// ┃┃ min_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ┣ ID you specified on the server
    /// ：
    /// ：
    /// ```
	pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
		self.since_id = Some(since_id.into());
		self
	}

    /// Set to get statuses that have ID greater than `min_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the oldest 20 statuses in between.
	/// 20 is the default value and can be changed with [`limit()`](#method.limit).
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃
    /// ┃┃ min_id=ID you specified
    /// ┃┃
    /// ┃┗
    /// ┣ ID you specified on the server
    /// ：
    /// ：
    /// ```
	pub fn min_id(mut self, min_id: impl Into<String>) -> Self {
		self.min_id = Some(min_id.into());
		self
	}

	/// Set a limit number of statuses to get.
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
	}
}

impl<'a> Method<'a, Statuses> for GetListTimeline<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::v1::lists;

    #[test]
    fn test_get_list_timeline() {
        let conn = Connection::new().unwrap();
        let list = lists::post(&conn, "mastorstestlistforgetlisttimeline").send().unwrap();

        assert!(
            get(&conn, list.id()).send().is_ok()
        );

        lists::delete(&conn, list.id()).send().unwrap();
    }
}
