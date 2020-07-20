use serde::Deserialize;
use crate::{
    DateTime,
    Utc,
};
use super::{
    Attachment,
    Entity,
    Nothing,
    Visibility,
};

/// Represents a status that will be published at a future scheduled date.
#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledStatus {
    // Required attributes
    id: String,
    scheduled_at: DateTime<Utc>,
    params: Box<Params>,
    media_attachments: Vec<Attachment>,
}

impl ScheduledStatus {
    /// Get an ID of this scheduled status in the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get a scheduled date and time of this scheduled status.
    pub fn scheduled_at(&self) -> &DateTime<Utc> {
        &self.scheduled_at
    }

    /// Get a params of this scheduled status.
    pub fn params(&self) -> &Params {
        &self.params
    }

    /// Get medias that is attached to this status.
    pub fn media_attachments(&self) -> &Vec<Attachment> {
        &self.media_attachments
    }
}

impl Entity for ScheduledStatus {}

/// Represents parameters of ScheduledStatus that will toot at scheduled date and time.
#[derive(Debug, Clone, Deserialize)]
pub struct Params {
    text: String,
    application_id: u64,
    visibility: Option<Visibility>,
    in_reply_to_id: Option<String>,
    media_ids: Option<Vec<String>>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    scheduled_at: Option<DateTime<Utc>>,
    poll: Option<ScheduledPoll>,
}

impl Params {
    /// Get a text of status that will posted at scheduled date and time.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get an application ID that used to create this scheduled status.
    pub fn application_id(&self) -> u64 {
        self.application_id
    }

    /// Get a visibility of status that will posted at scheduled date and time.
    pub fn visibility(&self) -> Option<Visibility> {
        self.visibility
    }

    /// Get a status ID that is this scheduled status will reply to.
    pub fn in_reply_to_id(&self) -> Option<&str> {
        self.in_reply_to_id.as_deref()
    }

    /// Get attached media IDs of this scheduled status.
    pub fn media_ids(&self) -> Option<&Vec<String>> {
        self.media_ids.as_ref()
    }

    /// Get whether the attachments of this scheduled status are sensitive.
    pub fn sensitive(&self) -> bool {
        if let Some(sensitive) = self.sensitive {
            sensitive
        } else {
            false
        }
    }

    /// Get a spoiler text of this scheduled status.
    pub fn spoiler_text(&self) -> Option<&str> {
        self.spoiler_text.as_deref()
    }

    /// Get a scheduled date and time of this scheduled status.
    pub fn scheduled_at(&self) -> Option<&DateTime<Utc>> {
        self.scheduled_at.as_ref()
    }

    pub fn poll(&self) -> Option<&ScheduledPoll> {
        self.poll.as_ref()
    }
}

/// Represents a poll that is containd in ScheduledStatus and will toot at scheduled date and time.
#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledPoll {
    multiple: bool,
    hide_totals: bool,
    expires_in: u64,
    options: Vec<String>,
}

impl ScheduledPoll {
    /// Get whether this poll allows multiple-choice votes.
    pub fn multiple(&self) -> bool {
        self.multiple
    }

    /// Get whether this poll do not show the total number of votes.
    pub fn hide_totals(&self) -> bool {
        self.hide_totals
    }

    /// Get the validity period of this poll as second.
    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    /// Get possible answers for the poll.
    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
}

/// Represent an array of [`ScheculedStatus`](./struct.ScheduledStatus.html)es.
pub type ScheduledStatuses = Vec<ScheduledStatus>;
impl Entity for ScheduledStatuses {}

/// Represents a no body response.
/// 
/// API method `DELETE /api/v1/scheduled_statuses/:id` returns nothing.
pub type DeletedScheduledStatus = Nothing;
