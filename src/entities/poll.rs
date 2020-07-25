use serde::Deserialize;
use crate::{
    DateTime,
    Utc,
};
use super::Emoji;

#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
/// Represents a poll attached to a status.
pub struct Poll {
    #[mastors(identifier)]
    id: String,

    expires_at: DateTime<Utc>,
    expired: bool,
    multiple: bool,
    votes_count: usize,
    voters_count: Option<usize>,
    voted: Option<bool>,
    own_votes: Option<Vec<u8>>,
    options: Vec<PollOption>,
    emojis: Vec<Emoji>,
}

impl Poll {
    /// Get the ID of the poll in the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the date time when the poll ends.
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Get whether the poll currently expired.
    pub fn expired(&self) -> bool {
        self.expired
    }

    /// Get whether the poll allows multiple-choice votes.
    pub fn multiple(&self) -> bool {
        self.multiple
    }

    /// Get the number of votes have been received.
    pub fn votes_count(&self) -> usize {
        self.votes_count
    }

    /// Get the number of unique accounts have voted if `multiple()` is `true`.
    pub fn voters_count(&self) -> Option<usize> {
        self.voters_count
    }

    /// Get whether voted if authorized user.
    pub fn voted(&self) -> Option<bool> {
        self.voted
    }

    /// Get indices of options which are your chosen if authorized user.
    pub fn own_votes(&self) -> Option<&Vec<u8>> {
        self.own_votes.as_ref()
    }

    /// Get possible answers for the poll.
    pub fn options(&self) -> &Vec<PollOption> {
        &self.options
    }

    /// Get custom emojis to be used for rendering poll options.
    pub fn emojis(&self) -> &Vec<Emoji> {
        &self.emojis
    }
}

/// One of the answers for the poll.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct PollOption {
    title: String,
    votes_count: Option<usize>,
}

impl PollOption {
    /// Get the text value of the poll option.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the number of votes for this option.
    pub fn votes_count(&self) -> Option<usize> {
        self.votes_count
    }
}
