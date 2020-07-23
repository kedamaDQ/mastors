//! This module provides features related to get the hashtag timeline.
use serde::Serialize;
use crate::{
	Connection,
	Method,
	entities::Statuses,
};

/// Get a request to get timeline only statuses contains tag specified by `hashtag`.
/// `hashtag` not including `#` symbol.
pub fn get(conn: &Connection, hashtag: impl Into<String>) -> GetTagTimeline {
    GetTagTimeline {
        conn,
        authorized: conn.public_timeline_preview_disabled(),
        hashtag: hashtag.into(),
        max_id: None,
        since_id: None,
        min_id: None,
        limit: None,
        local: None,
        only_media: None,
    }
}

/// GET request for `/api/v1/timelines/tag/:hashtag`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Statuses, "/api/v1/timelines/tag/_PATH_PARAM_")]
pub struct GetTagTimeline<'a> {
	#[serde(skip_serializing)]
	#[mastors(connection)]
	conn: &'a Connection,

	#[serde(skip_serializing)]
	#[mastors(authorization)]
	authorized: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    hashtag: String,

	max_id: Option<String>,
	since_id: Option<String>,
	min_id: Option<String>,
    limit: Option<usize>,

    local: Option<bool>,
    only_media: Option<bool>,
}

impl<'a> GetTagTimeline<'a> {
	/// Set the Authorization header to this GET request.
	/// Authorization header is force ON if `PUBLIC_TIMELINE_PREVIEW_DISABLED` is set in connection settings.
	pub fn authorized(mut self) -> Self {
		self.authorized = true;
		self
	}

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
    
	/// Set to get only statuses that are posted from the connected server.
	pub fn local(mut self) -> Self {
		self.local = Some(true);
		self
	}

	/// Set to get only statuses that are media attached.
	pub fn only_media(mut self) -> Self {
		self.only_media = Some(true);
		self
	}
}

impl<'a> Method<'a, Statuses> for GetTagTimeline<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::v1::statuses;

    #[test]
    fn test_get_hashtag_timeline() {
        let conn = Connection::new().unwrap();

        let posted = statuses::post(&conn, "test hashtag timeline #mastorstesthashtagmastorstesthashtag").send().unwrap().status().unwrap();
        let got = get(&conn, "mastorstesthashtagmastorstesthashtag").send().unwrap();
        assert!(! got.is_empty());
        assert_eq!(got.get(0).unwrap().id(), posted.id());

        statuses::id::delete(&conn, posted.id()).send().unwrap();
    }
}
