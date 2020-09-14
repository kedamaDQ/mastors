//! This module provides features related to status that are post a status, get a status, and reaction to status.
pub mod id;

use std::ops::Deref;
use chrono::{ DateTime, Duration, Utc };
use isolang::Language;
use serde::Serialize;
use crate::{
    Connection,
    Error,
    Method,
    Result,
    entities::{
        ScheduledStatus,
        Status,
        Visibility,
    },
};
use crate::synchronous::methods::private::MethodInternalWithoutRespHeader;

pub const LEAST_SCHEDULABLE_PERIOD: i64 = 302;

/// Get the base of POST request for `/api/v1/statuses`.
pub fn post(conn: &Connection) -> PostStatusesBase {
    PostStatusesBase {
        conn,
        language: conn.default_language().and_then(
            |lang| lang.to_639_1().map(|lang| lang.to_owned())
        ),
    }
}

/// Base parameters to post status.
/// 
/// This struct cannot send yet, add a status content text by `[status()](#status)` or attachment medias by `[media_ids()](#media_ids)` at least.
/// If you want to post the poll, set a status content text first.
pub struct PostStatusesBase<'a> {
    conn: &'a Connection,
    language: Option<String>,
}

impl<'a> PostStatusesBase<'a> {
    /// Add status content text to this POST request.
    pub fn status(self, status: impl AsRef<str>) -> PostStatusesSimple<'a> {
        PostStatusesSimple {
            conn: self.conn,
            auth: true,
            status: Some(status.as_ref().trim().to_string()),
            in_reply_to_id: None,
            spoiler_text: None,
            visibility: None,
            language: self.language,
        }
    }

    /// Add attachment medias to this POST request.
    pub fn media_ids<T, U>(self, media_ids: T) -> PostStatusesWithMediaAttachments<'a>
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        PostStatusesWithMediaAttachments {
            conn: self.conn,
            auth: true,
            inner: PostStatusesSimple {
                conn: self.conn,
                auth: true,
                status: None,
                in_reply_to_id: None,
                spoiler_text: None,
                visibility: None,
                language: self.language,
            },
            media_ids: MediaIds::new(media_ids, self.conn.status_max_medias()),
        }
    }

    pub fn poll<T, U>(self, status: impl AsRef<str>, options: T, expires_in: u64) -> PostStatusesWithPoll<'a>
    where
        T: AsRef<[U]>,
        U: AsRef<str>
    {
        let poll_max_options = self.conn.poll_max_options();

        PostStatusesWithPoll {
            conn: self.conn,
            auth: true,
            inner: self.status(status),
            poll: Poll::new(options, expires_in, poll_max_options),
        }
    }
}

/// POST request for `/api/v1/statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Status, "/api/v1/statuses")]
pub struct PostStatusesSimple<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    status: Option<String>,
    in_reply_to_id: Option<String>,
    spoiler_text: Option<String>,
    visibility: Option<Visibility>,
    language: Option<String>,
}

impl<'a> PostStatusesSimple<'a> {
    /// Add content text to this status.
    pub fn status(mut self, status: impl AsRef<str>) -> Self {
        let status = status.as_ref().trim();

        if !status.is_empty() {
            self.status = Some(status.to_string());
        }
        self
    }

    /// Add the status ID that is reply to. 
    pub fn in_reply_to_id(mut self, in_reply_to_id: impl AsRef<str>) -> Self {
        let in_reply_to_id = in_reply_to_id.as_ref().trim();

        if !in_reply_to_id.is_empty() {
            self.in_reply_to_id = Some(in_reply_to_id.to_string());
        }
        self
    }

    /// Add spoiler text to this status.
    pub fn spoiler_text(mut self, spoiler_text: impl AsRef<str>) -> Self {
        let spoiler_text = spoiler_text.as_ref().trim();

        if !spoiler_text.is_empty() {
            self.spoiler_text = Some(spoiler_text.to_string());
        }
        self
    }

    /// Set language to this status.
    pub fn language(mut self, language: impl AsRef<str>) -> Self {
        let language = language.as_ref().trim();

        if !language.is_empty() {
            self.language = Some(language.to_string());
        }
        self
    }

    /// Set the `Visibility` to this status.
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    /// Set status visibility to `public`.
    /// This is equivalent to `visibility(Visibility::Public)`.
    pub fn public(self) -> Self {
        self.visibility(Visibility::Public)
    }

    /// Set status visibility to `unlisted`.
    /// This is equivalent to `visibility(Visibility::Unlisted)`.
    pub fn unlisted(self) -> Self {
        self.visibility(Visibility::Unlisted)
    }

    /// Set status visibility to `private`.
    /// This is equivalent to `visibility(Visibility::Private)`.
    pub fn private(self) -> Self {
        self.visibility(Visibility::Private)
    }

    /// Set status visibility to `direct`.
    /// This is equivalent to `visibility(Visibility::Direct)`.
    pub fn direct(self) -> Self {
        self.visibility(Visibility::Direct)
    }

    /// Add media attachments to this status.
    pub fn media_ids<T, U>(self, media_ids: T) -> PostStatusesWithMediaAttachments<'a>
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        let status_max_characters = self.conn.status_max_characters();
        PostStatusesWithMediaAttachments {
            conn: self.conn,
            auth: true,
            inner: self,
            media_ids: MediaIds::new(media_ids, status_max_characters),
        }
    }

    /// Add poll to this status.
    pub fn poll<T, U>(self, options: T, expires_in: u64) -> PostStatusesWithPoll<'a>
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        PostStatusesWithPoll {
            conn: self.conn,
            auth: true,
            poll: Poll::new(options, expires_in, self.conn.poll_max_options()),
            inner: self,
        }
    }

    /// Set scheduled datetime to post this status.
    pub fn scheduled_at(self, scheduled_at: DateTime<Utc>) -> PostScheduledStatuses<'a> {
        PostScheduledStatuses {
            conn: self.conn,
            auth: true,
            inner: self,
            scheduled_at: ScheduledAt::new(scheduled_at),
        }
    }

    fn validate(&self) -> Result<()> {
        let mut total_chars: usize = 0;

        // Check status is not empty if set
        if let Some(status) = &self.status {
            if status.is_empty() {
                return Err(
                    Error::InvalidStatusError
                );
            }

            total_chars += status.chars().count();
        }

        // Check language if set
        if let Some(lang) = self.language.as_ref() {
            if Language::from_639_1(lang).is_none() {
                return Err(
                    Error::ParseIso639_1Error(lang.to_owned())
                );
            }
        }
    
        // Check total number of characters
        if let Some(spoiler_text) = self.spoiler_text.as_ref() {
            total_chars += spoiler_text.chars().count();
        }
    
        if total_chars > self.conn.status_max_characters() {
            return Err(
                Error::TooManyCharactersError(total_chars, self.conn.status_max_characters())
            );
        }
    
        Ok(())
    }
}

impl<'a> Method<'a, Status> for PostStatusesSimple<'a> {
    fn send(&self) -> Result<Status> {
        self.validate()?;
        self.send_internal()
    }
}

/// POST request with attachment medias for `/api/v1/statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Status, "/api/v1/statuses")]
pub struct PostStatusesWithMediaAttachments<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(flatten)]
    inner: PostStatusesSimple<'a>,

    #[serde(flatten)]
    media_ids: MediaIds,
}

impl<'a> PostStatusesWithMediaAttachments<'a> {
    /// Set scheduled datetime to post this status.
    pub fn scheduled_at(self, scheduled_at: DateTime<Utc>) -> PostScheduledStatusesWithMediaAttachments<'a> {
        PostScheduledStatusesWithMediaAttachments {
            conn: self.conn,
            auth: true,
            inner: self,
            scheduled_at: ScheduledAt::new(scheduled_at),
        }
    }

    fn validate(&self) -> Result<()> {
        self.inner.validate()?;
        self.media_ids.validate()?;
        Ok(())
    }
}

impl<'a> Deref for PostStatusesWithMediaAttachments<'a> {
    type Target = PostStatusesSimple<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Method<'a, Status> for PostStatusesWithMediaAttachments<'a> {
    fn send(&self) -> Result<Status> {
        self.validate()?;
        self.send_internal()
    }
}

/// POST request with a poll for `/api/v1/statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Status, "/api/v1/statuses")]
pub struct PostStatusesWithPoll<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(flatten)]
    inner: PostStatusesSimple<'a>,

    poll: Poll,
}

impl<'a> PostStatusesWithPoll<'a> {
    /// Set to hide total number of votes of poll.
    pub fn poll_hide_totals(mut self) -> Self {
        self.poll.hide_totals();
        self
    }

    /// Set to allow multiple vote for poll.
    pub fn poll_multiple(mut self) -> Self {
        self.poll.multiple();
        self
    }

    /// Set scheduled datetime to post this status.
    pub fn scheduled_at(self, scheduled_at: DateTime<Utc>) -> PostScheduledStatusesWithPoll<'a> {
        PostScheduledStatusesWithPoll{
            conn: self.conn,
            auth: true,
            inner: self,
            scheduled_at: ScheduledAt::new(scheduled_at),
        }
    }

    fn validate(&self) -> Result<()> {
        self.inner.validate()?;
        self.poll.validate()?;
        Ok(())
    }
}

impl<'a> Deref for PostStatusesWithPoll<'a> {
    type Target = PostStatusesSimple<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Method<'a, Status> for PostStatusesWithPoll<'a> {
    fn send(&self) -> Result<Status> {
        self.validate()?;
        self.send_internal()
    }
}

/// POST request for `/api/v1/scheduled_statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, ScheduledStatus, "/api/v1/statuses")]
pub struct PostScheduledStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(flatten)]
    inner: PostStatusesSimple<'a>,

    #[serde(flatten)]
    scheduled_at: ScheduledAt,
}

impl<'a> PostScheduledStatuses<'a> {
    fn validate(&self) -> Result<()> {
        self.inner.validate()?;
        self.scheduled_at.validate()?;
        Ok(())
    }
}

impl<'a> Deref for PostScheduledStatuses<'a> {
    type Target = PostStatusesSimple<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Method<'a, ScheduledStatus> for PostScheduledStatuses<'a> {
    fn send(&self) -> Result<ScheduledStatus> {
        self.validate()?;
        self.send_internal()
    }
}

/// POST request with attachment medias for `/api/v1/scheduled_statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, ScheduledStatus, "/api/v1/statuses")]
pub struct PostScheduledStatusesWithMediaAttachments<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(flatten)]
    inner: PostStatusesWithMediaAttachments<'a>,

    #[serde(flatten)]
    scheduled_at: ScheduledAt,
}

impl<'a> PostScheduledStatusesWithMediaAttachments<'a> {
    fn validate(&self) -> Result<()> {
        self.inner.validate()?;
        self.scheduled_at.validate()?;
        Ok(())
    }
}

impl<'a> Deref for PostScheduledStatusesWithMediaAttachments<'a> {
    type Target = PostStatusesWithMediaAttachments<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Method<'a, ScheduledStatus> for PostScheduledStatusesWithMediaAttachments<'a> {
    fn send(&self) -> Result<ScheduledStatus> {
        self.validate()?;
        self.send_internal()
    }
}

/// POST request with a poll for `/api/v1/scheduled_statuses`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, ScheduledStatus, "/api/v1/statuses")]
pub struct PostScheduledStatusesWithPoll<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(flatten)]
    inner: PostStatusesWithPoll<'a>,

    #[serde(flatten)]
    scheduled_at: ScheduledAt,
}

impl<'a> PostScheduledStatusesWithPoll<'a> {
    pub fn poll_hide_totals(mut self) -> Self {
        self.inner.poll.hide_totals();
        self
    }

    pub fn poll_multiple(mut self) -> Self {
        self.inner.poll.multiple();
        self
    }

    fn validate(&self) -> Result<()> {
        self.inner.validate()?;
        self.scheduled_at.validate()?;
        Ok(())
    }
}

impl<'a> Deref for PostScheduledStatusesWithPoll<'a> {
    type Target = PostStatusesWithPoll<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Method<'a, ScheduledStatus> for PostScheduledStatusesWithPoll<'a> {
    fn send(&self) -> Result<ScheduledStatus> {
        self.validate()?;
        self.send_internal()
    }
}

/// Wrapper for the media_ids.
#[derive(Debug, Clone, Serialize)]
struct MediaIds {
    media_ids: Vec<String>,

    #[serde(skip_serializing)]
    status_max_medias: usize,
}

impl MediaIds {
    fn new<T, U>(media_ids: T, status_max_medias: usize) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        let media_ids = media_ids.as_ref()
            .iter()
            .map(|u| u.as_ref().trim())
            .filter(|u| !u.is_empty())
            .map(|u| u.to_owned())
            .collect::<Vec<String>>();

        MediaIds {
            media_ids,
            status_max_medias,
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.media_ids.is_empty()
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.media_ids.len()
    }

    fn validate(&self) -> Result<()> {
        use std::collections::HashSet;

        if self.media_ids.is_empty() {
            return Err(Error::NoAttachmentMediaError);
        }

        if self.media_ids.len() > self.status_max_medias {
            return Err(
                Error::TooManyAttachmentMediasError(self.media_ids.len(), self.status_max_medias)
            );
        }

        if self.media_ids.iter().collect::<HashSet<&String>>().len() != self.media_ids.len() {
            return Err(
                Error::DuplicateMediaError
            );
        }

        Ok(())
    }
}

/*
impl serde::ser::Serialize for MediaIds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for media_id in self.media_ids.iter() {
            seq.serialize_element(&media_id)?;
        }
        seq.end()
    }
}
*/

/// Poll options.
#[derive(Debug, Clone, Serialize)]
struct Poll {
    options: Vec<String>,
    expires_in: u64,
    multiple: bool,
    hide_totals: bool,

    #[serde(skip_serializing)]
    max_options: usize,
}

impl Poll {
    fn new<T, U>(options: T, expires_in: u64, max_options: usize) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        let options = options.as_ref()
            .iter()
            .map(|u| u.as_ref().trim())
            .filter(|u| !u.is_empty())
            .map(|u| u.to_owned())
            .collect::<Vec<String>>();

        Poll {
            options,
            expires_in,
            multiple: false,
            hide_totals: false,
            max_options,
        }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.options.len()
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    fn multiple(&mut self) {
        self.multiple = true;
    }

    fn hide_totals(&mut self) {
        self.hide_totals = true;
    }

    fn validate(&self) -> Result<()> {
        use std::collections::HashSet;

        if self.options.len() < 2 {
            return Err(
                Error::TooLittlePollOptionsError
            );
        }

        if self.options.len() > self.max_options {
            return Err(
                Error::TooManyPollOptionsError(self.options.len(), self.max_options)
            );
        }

        if self.options.iter().collect::<HashSet<&String>>().len() != self.options.len() {
            return Err(
                Error::DuplicatePollOptionError
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
struct ScheduledAt {
    scheduled_at: DateTime<Utc>,
}

impl ScheduledAt {
    fn new(scheduled_at: DateTime<Utc>) -> Self {
        ScheduledAt {
            scheduled_at,
        }
    }

    fn validate(&self) -> Result<()> {
        let now = Utc::now();

        if self.scheduled_at - now < Duration::seconds(LEAST_SCHEDULABLE_PERIOD) {
            return Err(
                Error::ScheduleTooCloseError(now, self.scheduled_at)
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statuses() {
        let conn = Connection::new().unwrap();
        let content = body("toot!");
        let posted = post(&conn)
            .status(&content)
            .spoiler_text("spoiler text")
            .unlisted()
            .private()
            .direct()
            .public()
            .language("ja")
            .send()
            .unwrap();

        let got = super::id::get(&conn, posted.id())
            .authorized()
            .unauthorized()
            .authorized()
            .send()
            .unwrap();

        assert_eq!(posted.id(), got.id());

        let deleted = super::id::delete(&conn, posted.id())
            .send()
            .unwrap();

        assert_eq!(posted.id(), deleted.id());
        assert_eq!(&content, deleted.text().unwrap());
    }

    #[test]
    fn test_statuses_with_poll() {
        let conn = Connection::new().unwrap();
        let content = body("with poll!");
        let posted = post(&conn)
            .poll(&content, &(vec!["poll1", "poll2", "poll3"]), 3600)
            .poll_multiple()
            .poll_hide_totals()
            .send()
            .unwrap();

        let got = super::id::get(&conn, posted.id())
            .authorized()
            .send()
            .unwrap();

        assert_eq!(posted.id(), got.id());
        assert_eq!(posted.poll().unwrap().id(), got.poll().unwrap().id());

        let deleted = super::id::delete(&conn, posted.id())
            .send()
            .unwrap();

        assert_eq!(got.id(), deleted.id());
        assert_eq!(&content, deleted.text().unwrap());
        assert_eq!(got.poll().unwrap().id(), deleted.poll().unwrap().id())
    }

    #[test]
    fn test_status_with_attachment() {
        use crate::api::v1::media;

        let conn = Connection::new().unwrap();
        let content = body("with attachment!");

        let media_ids = vec![
            media::post(&conn, "./test-resources/test1.png").send().unwrap().id().to_owned(),
            media::post(&conn, "./test-resources/test2.png").send().unwrap().id().to_owned(),
        ];

        let posted = post(&conn)
            .status(&content)
            .media_ids(&media_ids)
            .send()
            .unwrap();

        let got = super::id::get(&conn, posted.id())
            .send()
            .unwrap();

        let got_media_ids = got
            .media_attachments()
            .iter()
            .map(|ma| ma.id().to_owned())
            .collect::<Vec<String>>();

        assert_eq!(posted.id(), got.id());
        assert_eq!(&media_ids, &got_media_ids);

        let deleted = super::id::delete(&conn, got.id())
            .send()
            .unwrap();

        assert_eq!(got.id(), deleted.id());
        assert_eq!(
            media_ids,
            deleted.media_attachments()
                .iter()
                .map(|ma| ma.id().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_scheduled_status() {
        let conn = Connection::new().unwrap();
        let scheduled_at = Utc::now() + chrono::Duration::seconds(310);

        let posted = post(&conn)
            .status(body("scheduled"))
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();

        // Mastodon rounds down nano-secs, following:
        //  left: `2020-05-26T11:11:34.730Z`'
        //  right: `2020-05-26T11:11:34.730011387Z`,
//      assert_eq!(posted.scheduled_at().clone(), scheduled_at);

        let got = crate::api::v1::scheduled_statuses::id::get(&conn, posted.id())
            .send()
            .unwrap();

        assert_eq!(posted.id(), got.id());
        assert_eq!(posted.scheduled_at(), got.scheduled_at());

        let extended_scheduled_at = got.scheduled_at() + chrono::Duration::seconds(100);
        let put = crate::api::v1::scheduled_statuses::id::put(&conn, got.id())
            .scheduled_at(extended_scheduled_at)
            .send()
            .unwrap();

        assert_eq!(got.id(), put.id());
        assert_eq!(put.scheduled_at().clone(), extended_scheduled_at);

        let _deleted = crate::api::v1::scheduled_statuses::id::delete(&conn, put.id())
            .send()
            .unwrap();

        let got = crate::api::v1::scheduled_statuses::id::get(&conn, got.id())
            .send();

        assert!(got.is_err());
    }

    #[test]
    fn test_scheduled_status_with_media() {
        let conn = Connection::new().unwrap();
        let scheduled_at = Utc::now() + chrono::Duration::seconds(310);

        let media_ids = vec![
            crate::api::v1::media::post(&conn, "./test-resources/test1.png").send().unwrap().id().to_owned(),
            crate::api::v1::media::post(&conn, "./test-resources/test2.png").send().unwrap().id().to_owned(),
        ];

        let posted = post(&conn)
            .media_ids(&media_ids)
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();

        let _deleted = crate::api::v1::scheduled_statuses::id::delete(&conn, posted.id())
            .send()
            .unwrap();

        let got = crate::api::v1::scheduled_statuses::id::get(&conn, posted.id())
            .send();

        assert!(got.is_err());
    }

    #[test]
    fn test_scheduled_status_with_poll() {
        let conn = Connection::new().unwrap();
        let scheduled_at = Utc::now() + chrono::Duration::seconds(310);

        let posted = post(&conn)
            .status("scheduled status with poll")
            .poll(&["a", "b"], 3600)
            .scheduled_at(scheduled_at)
            .poll_hide_totals()
            .poll_multiple()
            .send()
            .unwrap();

        let _deleted = crate::api::v1::scheduled_statuses::id::delete(&conn, posted.id())
            .send()
            .unwrap();

        let got = crate::api::v1::scheduled_statuses::id::get(&conn, posted.id())
            .send();
        
        assert!(got.is_err());
    }

    #[test]
    fn test_media_ids_construction() {
        let ids = ["", "", "a", "b", "c"];

        // &[str]
        let media_ids = MediaIds::new(&ids, 4);
        assert_eq!(media_ids.len(), 3);

        // [str]
        let media_ids = MediaIds::new(ids, 4);
        assert_eq!(media_ids.len(), 3);

        let ids = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), String::new()];

        // &Vec<String>
        let media_ids = MediaIds::new(&ids, 4);
        assert_eq!(media_ids.len(), 3);

        // Vec<String>
        let media_ids = MediaIds::new(ids, 4);
        assert_eq!(media_ids.len(), 3);

        let ids = vec!["a", "b", "", "c"];

        // &Vec<&str>
        let media_ids = MediaIds::new(&ids, 4);
        assert_eq!(media_ids.len(), 3);

        // Vec<&str>
        let media_ids = MediaIds::new(ids, 4);
        assert_eq!(media_ids.len(), 3);
    }

    #[test]
    fn test_media_ids_validation() {
        let ids: Vec<String> = Vec::new();

        // no id
        let media_ids = MediaIds::new(ids, 4);
        assert!(media_ids.validate().is_err());

        // too many ids
        let media_ids = MediaIds::new(["a", "b", "c", "d", "", "e"], 4);
        assert!(media_ids.validate().is_err());

        // id duplication
        let media_ids = MediaIds::new(["a", "b", "c", "a"], 4);
        assert!(media_ids.validate().is_err());
    }

    #[test]
    fn test_poll_construction() {
        let options = ["", "", "a", "b", "c"];

        // &[str]
        let poll = Poll::new(&options, 3600, 4);
        assert_eq!(poll.len(), 3);

        // [str]
        let poll = Poll::new(options, 3600, 4);
        assert_eq!(poll.len(), 3);

        let options = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), String::new()];

        // &Vec<String>
        let poll = Poll::new(&options, 3600, 4);
        assert_eq!(poll.len(), 3);

        // Vec<String>
        let poll = Poll::new(options, 3600, 4);
        assert_eq!(poll.len(), 3);

        let options = vec!["a", "b", "", "c"];

        // &Vec<&str>
        let poll = Poll::new(&options, 3600, 4);
        assert_eq!(poll.len(), 3);

        // Vec<&str>
        let poll = Poll::new(options, 3600, 4);
        assert_eq!(poll.len(), 3);
    }

    #[test]
    fn test_poll_validation() {
        let options: Vec<String> = Vec::new();

        // no option
        let poll = Poll::new(options, 3600, 4);
        assert!(poll.validate().is_err());

        // too little options
        let poll = Poll::new(["a"], 3600, 4);
        assert!(poll.validate().is_err());

        // too many options
        let poll = Poll::new(["a", "b", "c", "d", "", "e"], 3600, 4);
        assert!(poll.validate().is_err());

        // option duplication
        let poll = Poll::new(["a", "b", "c", "", "a"], 3600, 4);
        assert!(poll.validate().is_err());
    }

    fn body(s: &str) -> String {
        "Test ".to_string() + s + "\n\n" + chrono::Local::now().to_rfc3339().as_str()
    }
}
