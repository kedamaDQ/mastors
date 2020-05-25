//pub mod id;

use serde::Serialize;
use crate::{
    Connection,
    DateTime,
    Error,
    Language,
    Result,
    Utc,
    entities::{
        Status,
        Visibility,
    },
    methods::{
        Method,
        MethodInternal,
    },
};

/// Create a request to get a status specified by `id`.
pub fn get(conn: &Connection, id: impl Into<String>) -> GetStatuses {
    GetStatuses {
        conn,
        id: id.into(),
        authorized: true,
    }
}

/// Create a request to post the status.
pub fn post(
    conn: &Connection,
    status: impl AsRef<str>,
) -> PostStatuses {

    post_inner(conn, str_to_option(status), None, None)
}

/// Create a request to post the status with attached medias.
/// 
/// If you want to create the status without the text, set `status` to an empty string such as `""`.
pub fn post_with_media<T, U>(
    conn: &Connection,
    status: impl AsRef<str>,
    media_ids: T,
) -> PostStatuses
where
    T: AsRef<[U]>,
    U: AsRef<str>,
{

    post_inner(
        conn,
        str_to_option(status),
        Some(MediaIds::new(media_ids, conn.status_max_medias())),
        None
    )
}

/// Create a request to post the status with poll.
pub fn post_with_poll<T, U>(
    conn: &Connection,
    status: impl AsRef<str>,
    poll_options: T,
    poll_expires_in: u64,
) -> PostStatuses
where
    T: AsRef<[U]>,
    U: AsRef<str>,
{
    post_inner(
        conn,
        str_to_option(status),
        None,
        Some(Poll::new(poll_options, poll_expires_in, conn.poll_max_options()))
    )
}

pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteStatuses {
    DeleteStatuses {
        conn,
        auth: true,
        id: id.into(),
    }
}

/// GET request for /api/v1/statuses/:id
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(GET, Status, "/api/v1/statuses/_PATH_PARAM_")]
pub struct GetStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,
}

impl<'a> GetStatuses<'a> {
    /// Add Authorization header to GET request.
    pub fn authorized(mut self) -> Self {
        self.authorized = true;
        self
    }

    /// Remove Authorization header from GET request.
    pub fn unauthorized(mut self) -> Self {
        self.authorized = false;
        self
    }
}

impl<'a> Method<'a, Status> for GetStatuses<'a> {}

// Create POST request.
fn post_inner(
    conn: &Connection,
    status: Option<String>,
    media_ids: Option<MediaIds>,
    poll: Option<Poll>,
) -> PostStatuses {

    PostStatuses {
        conn,
        auth: true,
        status,
        media_ids,
        poll,
        in_reply_to_id: None,
        sensitive: None,
        spoiler_text: None,
        visibility: None,
        scheduled_at: None,
        language: conn.default_language().and_then(|lang| lang.to_639_1()),
    }
}

/// POST request for /api/v1/statuses
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(POST, Status, "/api/v1/statuses")]
pub struct PostStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    status: Option<String>,
    media_ids: Option<MediaIds>,
    poll: Option<Poll>,
    in_reply_to_id: Option<String>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    visibility: Option<String>,
    scheduled_at: Option<DateTime<Utc>>,
    language: Option<&'a str>,
}

impl<'a> PostStatuses<'a> {
    /// Add an in_reply_to_id to status.
    pub fn in_reply_to_id(mut self, id: impl AsRef<str>) -> Self {
        self.in_reply_to_id = str_to_option(id);
        self
    }

    /// Set status to sensitive if media_ids is set.
    pub fn sensitive(mut self) -> Self {
        if self.media_ids.is_some() {
            self.sensitive = Some(true);
        }
        self
    }

    /// Add a spoiler_text to status.
    pub fn spoiler_text(mut self, spoiler_text: impl AsRef<str>) -> Self {
        self.spoiler_text = str_to_option(spoiler_text);
        self
    }

    /// Set the `Visibility` to status.
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility.to_string());
        self
    }

    /// Set status visibility to `public`.
    /// This is equivalent to `visibility(Visibility::Public)`.
    pub fn public(mut self) -> Self {
        self.visibility = Some(Visibility::Public.to_string());
        self
    }

    /// Set status visibility to `unlisted`.
    /// This is equivalent to `visibility(Visibility::Unlisted)`.
    pub fn unlisted(mut self) -> Self {
        self.visibility = Some(Visibility::Unlisted.to_string());
        self
    }
    
    /// Set status visibility to `private`.
    /// This is equivalent to `visibility(Visibility::Private)`.
    pub fn private(mut self) -> Self {
        self.visibility = Some(Visibility::Private.to_string());
        self
    }

    /// Set status visibility to `direct`.
    /// This is equivalent to `visibility(Visibility::Direct)`.
    pub fn direct(mut self) -> Self {
        self.visibility = Some(Visibility::Direct.to_string());
        self
    }

    /// Set a status to scheduled.
    pub fn scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }

    /// Set language to status.
    pub fn language(mut self, language: Language) -> Self {
        self.language = language.to_639_1();
        self
    }

    /// Set to allow multiple choices if poll is present.
    pub fn poll_multiple(mut self) -> Self {
        self.poll = self.poll.map(|p| p.multiple());
        self
    }

    /// Set to hide number of total votes if poll is present.
    pub fn poll_hide_totals(mut self) -> Self {
        self.poll = self.poll.map(|p| p.hide_totals());
        self
    }
}

impl<'a> Method<'a, Status> for PostStatuses<'a> {
    /// Send a status to the server.
    /// 
    /// # Errors
    /// 
    /// This method will return the error if:
    /// 
    /// - Both of `status` and media_ids are nothing
    /// - `media_ids` is empty or contains number of elements more than `STATUS_MAX_MEDIAS`
    /// - `media_ids` contains duplicate media_id
    /// - `scheduled_at` is set a date time in the past
    /// - `poll_options` contains options less than 2 or more than `POLL_MAX_OPTIONS`
    /// - `poll_options` contains duplicate option
    /// - Total number of characters of `status` and `spoiler_text` exceeds `STATUS_MAX_CHARACTERS`
    fn send(&'a self) -> Result<Status> {

        if self.media_ids.is_some() && self.poll.is_some() {
            panic!("Cannot attach both media and poll.");
        }
    
        if self.status.is_none() && self.media_ids.is_none() {
            return Err(
                Error::InvalidStatusError("There is neither status nor media".to_owned())
            );
        }
 
        // Check media_ids
        if let Some(media_ids) = &self.media_ids {
            media_ids.validate()?;
        }

        // Check shceduled date time is future
        if let Some(scheduled_at) = self.scheduled_at {
            if scheduled_at < Utc::now() {
                return Err(Error::PastDateTimeError(scheduled_at));
            }
        }

        // Check poll options
        if let Some(poll) = &self.poll {
            poll.validate()?;
        }

        // Check number of chars.
        let mut total_chars: usize = 0;

        if let Some(status) = &self.status {
            total_chars += status.len();
        }

        if let Some(spoiler_text) = &self.spoiler_text {
            total_chars += spoiler_text.len();
        }

        if total_chars > self.conn.status_max_characters() {
            return Err(
                Error::TooManyCharactersError(total_chars, self.conn.status_max_characters())
            );
        }

        Ok(self.send_internal()?)
    }
}

/// DELETE request for /api/v1/statuses/:id
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(DELETE, Status, "/api/v1/statuses/_PATH_PARAM_")]
pub struct DeleteStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,
}

impl<'a> Method<'a, Status> for DeleteStatuses<'a> {}

/// Wrapper for media_ids.
#[derive(Debug, Clone)]
struct MediaIds {
    media_ids: Vec<String>,
    status_max_medias: usize,
}

impl MediaIds {
    fn new<T, U>(media_ids: T, status_max_medias: usize) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        let media_ids = media_ids.as_ref()
            .into_iter()
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

/// Poll options.
#[derive(Debug, Serialize)]
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
            .into_iter()
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

    // Set this poll to be able to multiple votes.
    fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    // Set this poll to do not show total number of votes.
    fn hide_totals(mut self) -> Self {
        self.hide_totals = true;
        self
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

fn str_to_option(s: impl AsRef<str>) -> Option<String> {
    let s = s.as_ref().trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Local;

    const ENV_TEST: &str = ".env.test";

    #[test]
    fn test_statuses() {
        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        let content = body("toot!");
        let posted = post(&conn, &content)
            .spoiler_text("spoiler text")
            .unlisted()
            .private()
            .direct()
            .public()
            .send()
            .unwrap();

        let got = get(&conn, posted.id())
            .authorized()
            .unauthorized()
            .authorized()
            .send()
            .unwrap();

        assert_eq!(posted.id(), got.id());

        let deleted = delete(&conn, posted.id())
            .send()
            .unwrap();
        
        assert_eq!(posted.id(), deleted.id());
        assert_eq!(&content, deleted.text().unwrap());
    }

    #[test]
    fn test_statuses_with_poll() {
        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        let content = body("with poll!");
        let posted = post_with_poll(&conn, &content, &(vec!["poll1", "poll2", "poll3"]), 3600)
            .poll_multiple()
            .poll_hide_totals()
            .send()
            .unwrap();

        let got = get(&conn, posted.id())
            .authorized()
            .send()
            .unwrap();
        
        assert_eq!(posted.id(), got.id());
        assert_eq!(posted.poll().unwrap().id(), got.poll().unwrap().id());

        let deleted = delete(&conn, posted.id())
            .send()
            .unwrap();
        
        assert_eq!(got.id(), deleted.id());
        assert_eq!(&content, deleted.text().unwrap());
        assert_eq!(got.poll().unwrap().id(), deleted.poll().unwrap().id())
    }

    #[test]
    fn test_status_with_attachment() {
        use crate::api::v1::media;

        let conn = Connection::new_with_path(ENV_TEST).unwrap();
        let content = body("with attachment!");

        let media_ids = vec![
            media::post(&conn, "./test-resources/test1.png").send().unwrap().id().to_owned(),
            media::post(&conn, "./test-resources/test2.png").send().unwrap().id().to_owned(),
        ];

        let posted = post_with_media(&conn, &content, &media_ids)
            .send()
            .unwrap();

        let got = get(&conn, posted.id())
            .send()
            .unwrap();
        
        let got_media_ids = got
            .media_attachments()
            .iter()
            .map(|ma| ma.id().to_owned())
            .collect::<Vec<String>>();

        assert_eq!(posted.id(), got.id());
        assert_eq!(&media_ids, &got_media_ids);

        let deleted = delete(&conn, got.id())
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
        "Test ".to_string() + s + "\n\n" + Local::now().to_rfc3339().as_str()
    }
}
