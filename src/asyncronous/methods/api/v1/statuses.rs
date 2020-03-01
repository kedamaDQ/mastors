use async_trait::async_trait;
use serde::Serialize;
use crate::{
    Connection,
    DateTime,
    Error,
    Result,
    Utc,
    entities::{
        Status,
        Visibility,
    },
    methods::Method,
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
/// 
/// # Errors
/// 
/// This function will return an error if `status` is empty or blank.
pub fn post(
    conn: &Connection,
    status: impl Into<String>,
) -> Result<PostStatuses> {

    let status = status.into().trim().to_string();
    if status.is_empty() {
        return Err(Error::EmptyStatusError);
    }

    post_inner(conn, Some(status), None, None)
}

/// Create a request to post the status with attached medias.
/// 
/// If you want to create the status without the text, set `status` to an empty string such as `""`.
/// 
/// # Errors
/// 
/// This function will return an error if `media_ids` is empty.
pub fn post_with_media(
    conn: &Connection,
    status: impl Into<String>,
    media_ids: Vec<String>
) -> Result<PostStatuses> {

    let media_ids: Vec<String> = media_ids.iter()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    if media_ids.is_empty() {
        return Err(Error::NoAttachmentMediaError);
    }

    if media_ids.len() > conn.status_max_medias() {
        return Err(
            Error::TooManyAttachmentMediasError(media_ids.len(), conn.status_max_medias())
        );
    }

    // If the parameter `status` is defined as Option<impl Into<String>>.
    // It is possible, but cannot use `None` in actual argument when call this function...
    /*
    let status = status
        .map(|s| s.into().trim().to_string())
        .filter(|s| !s.is_empty());
    */
    let status = status.into().trim().to_string();
    let status = if status.is_empty() {
        None
    } else {
        Some(status)
    };

    post_inner(conn, status, Some(media_ids), None)
}

/// Create a request to post the status with poll.
/// 
/// # Errors
/// 
/// This function will return an error if `status` is empty or blank.
pub fn post_with_poll(
    conn: &Connection,
    status: impl Into<String>,
    poll: Poll
) -> Result<PostStatuses> {

    let status = status.into().trim().to_string();
    if status.is_empty() {
        return Err(Error::EmptyStatusError);
    }

    post_inner(conn, Some(status), None, Some(poll))
}

/// Construct a new `Poll`.
/// 
/// # Errors
/// This function will return an error if `options` is empty or all element of options are blank string or if number of options more than `POLL_MAX_OPTIONS` environment variables.
/// 
pub fn create_poll(conn: &Connection, options: Vec<&str>, expires_in: u64) -> Result<Poll> {
    let options: Vec<String> = options
        .iter()
        .map(|o| o.trim())
        .filter(|o| !o.is_empty())
        .map(|o| o.to_owned())
        .collect();

    if options.len() < 2 {
        return Err(Error::TooLittlePollOptionsError);
    }

    if options.len() > conn.poll_max_options() {
        return Err(Error::TooManyPollOptionsError(options.len(), conn.poll_max_options()));
    }

    Ok(Poll {
        options,
        expires_in,
        multiple: false,
        hide_totals: false,
    })
}

// Create POST request.
fn post_inner(
    conn: &Connection,
    status: Option<String>,
    media_ids: Option<Vec<String>>,
    poll: Option<Poll>,
) -> Result<PostStatuses> {

    if media_ids.is_some() && poll.is_some() {
        panic!("Cannot attach both media and poll.");
    }

    if status.is_none() && media_ids.is_none() {
        panic!("Status requires status or media_ids.");
    }

    Ok(PostStatuses {
        conn,
        status,
        media_ids,
        poll,
        in_reply_to_id: None,
        sensitive: None,
        spoiler_text: None,
        visibility: None,
        scheduled_at: None,
        language: conn.default_language(),
    })
}

/// GET request for /api/v1/statuses/:id
#[derive(Debug, Serialize)]
pub struct GetStatuses<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    authorized: bool,

    #[serde(skip_serializing)]
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

#[async_trait]
impl<'a> Method<'a, Status> for GetStatuses<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses";

    fn path(&self) -> String {
        format!("{}/{}", Self::ENDPOINT, &self.id)
    }

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization_code(&self) -> Option<&str> {
        if self.authorized {
            Some(self.conn.access_token())
        } else {
            None
        }
    }

    async fn send(&'a self) -> Result<Status> {
        Ok(self.get().await?)
    }
}

/// POST request for /api/v1/statuses
#[derive(Debug, Serialize)]
pub struct PostStatuses<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    status: Option<String>,
    media_ids: Option<Vec<String>>,
    poll: Option<Poll>,
    in_reply_to_id: Option<String>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    visibility: Option<String>,
    scheduled_at: Option<DateTime<Utc>>,
    language: Option<&'static str>,
}

impl<'a> PostStatuses<'a> {
    /// Add an in_reply_to_id to status.
    pub fn in_reply_to_id(mut self, id: impl Into<String>) -> Self {
        self.in_reply_to_id = Some(id.into());
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
    pub fn spoiler_text(mut self, spoiler_text: impl Into<String>) -> Self {
        let spoiler_text = spoiler_text.into();
        if !spoiler_text.is_empty() {
            self.spoiler_text = Some(spoiler_text);
        }
        self
    }

    /// Set the `Visibility` to status.
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility.to_string());
        self
    }

    /// Set a status to scheduled.
    /// 
    /// # Errors
    /// 
    /// This method will return an error if `scheduled_at` is pasted.
    pub fn scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Result<Self> {
        if scheduled_at < Utc::now() {
            return Err(Error::PastDateTimeError(scheduled_at));
        }

        self.scheduled_at = Some(scheduled_at);
        Ok(self)
    }

    /// Set language to status.
    /// 
    /// Language must be ISO639-1 compliant.
    /// 
    /// # Errors
    /// 
    /// This method will return an error if `language` is not ISO639-1 compliant.
    /// This method will ignore calling if `language` is not IOS639-1 compliant.
    pub fn language(mut self, language: impl Into<String>) -> Result<Self> {
        use isolang::Language;

        let language = language.into();
        self.language = Language::from_639_1(&language)
            .ok_or_else(|| Error::ParseIso639_1Error(language))?
            .to_639_1();
        Ok(self)
    }
}

#[async_trait]
impl<'a> Method<'a, Status> for PostStatuses<'a> {
    const ENDPOINT: &'a str = "/api/v1/statuses";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization_code(&self) -> Option<&str> {
        Some(self.conn.access_token())
    }

    async fn send(&'a self) -> Result<Status> {
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

        Ok(self.post().await?)
    }
}

/// Poll options.
#[derive(Debug, Serialize)]
pub struct Poll {
    options: Vec<String>,
    expires_in: u64,
    multiple: bool,
    hide_totals: bool,
}

impl Poll {
    /// Set this poll to be able to multiple votes.
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Set this poll to do not show total number of votes.
    pub fn hide_totals(mut self) -> Self {
        self.hide_totals = true;
        self
    }
}
