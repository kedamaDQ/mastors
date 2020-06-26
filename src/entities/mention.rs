use serde::Deserialize;
use crate::Url;
use super::Entity;

/// Represents a mention of a user within the content of a status.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct Mention {
    // Required attributes
    id: String,
    username: String,
    url: Url,
    acct: String,
}

impl Mention {
    /// Get the account id of the mentioned user.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the username of the mentioned user.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the location of the mentioned user's profile.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the webfinger acct: URI of the mentioned user. Equivalent to `username` for local users, or `username@domain` for remote users.
    pub fn acct(&self) -> &str {
        &self.acct
    }
}

impl Entity for Mention {}
