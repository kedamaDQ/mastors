//! This module provides features related to statuses posted by account specified by ID.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::Statuses,
};

/// Get a request to get statuses posted by account specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetStatuses {
    GetStatuses {
        conn,
        id: id.into(),
        authorized: conn.whitelist_mode(),
        max_id: None,
        min_id: None,
        since_id: None,
        tagged: None,
        pinned: None,
        only_media: None,
        exclude_replies: None,
        exclude_reblogs: None,
    }
}

/// Get request to get statuses posted by account specified by ID.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Statuses, "/api/v1/accounts/_PATH_PARAM_/statuses")]
pub struct GetStatuses<'a> {
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
    min_id: Option<String>,
    since_id: Option<String>,
    tagged: Option<String>,
    pinned: Option<bool>,
    only_media: Option<bool>,
    exclude_reblogs: Option<bool>,
    exclude_replies: Option<bool>,

}

impl<'a> GetStatuses<'a> {
    /// Set `Authorization` header to this request.
    /// 
    /// If set to false, server returns only public statuses.
    /// If server is whitelist mode and set to false, request will fail with 401 `Unauthorized`.
    pub fn authorized(mut self, authorized: bool) -> Self {
        self.authorized = authorized;
        self
    }

    /// Set to get statuses that have ID less than `max_id`.
    pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    /// Set to get statuses that have ID greater than `min_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the oldest 20 statuses in between.
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃ min_id=ID you specified
    /// ┃┃
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

    /// Set to get latest statuses that have ID greater than `since_id`.
    /// 
    /// If an ID you specify is more than 20 older than the latest status on the server, this method gets the latest 20 statuses in between.
    /// 
    /// ```text
    /// ┏ latest status ID on the server
    /// ┃┏
    /// ┃┃ since_id=ID you specified
    /// ┃┃
    /// ┃┃
    /// ┃┗
    /// ：
    /// ：
    /// ┃┏
    /// ┃┃ min_id=ID you specified
    /// ┃┃
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

    /// Set filter for statuses with hashtag.
    pub fn tagged(mut self, tagged: impl Into<String>) -> Self {
        self.tagged = Some(tagged.into());
        self
    }

    /// Set filter only for pinned statuses.
    pub fn pinned(mut self) -> Self {
        self.pinned = Some(true);
        self
    }

    /// Set filter only for statuses with attached media.
    pub fn only_media(mut self) -> Self {
        self.only_media = Some(true);
        self
    }

    /// Set filter to exclude replies.
    pub fn exclude_replies(mut self) -> Self {
        self.exclude_replies = Some(true);
        self
    }

    /// Set filter to exclude reblogs.
    pub fn exclude_reblogs(mut self) -> Self {
        self.exclude_reblogs = Some(true);
        self
    }
}

impl<'a> Method<'a, Statuses> for GetStatuses<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_statuses() {
        let conn = Connection::new().unwrap();
        get(&conn, id()).send().unwrap();
    }

    use crate::api::v1::accounts::verify_credentials;
    fn id() -> String {
        let conn = Connection::new().unwrap();
        verify_credentials::get(&conn).send().unwrap().id().to_owned()
    }

}
