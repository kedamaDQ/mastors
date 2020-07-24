use serde::Deserialize;
use crate::Url;
use super::{
    Account,
    Entity,
};

/// Represents the software instance of Mastodon running on this domain.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, mastors_derive::Entity)]
pub struct Instance {
    uri: String,
    title: String,
    description: String,
    short_description: String,
    email: String,
    version: String,
    languages: Vec<String>,
    registrations: bool,
    approval_required: bool,
    urls: Urls,
    stats: Stats,
    thumbnail: Option<Url>,
    contact_account: Option<Account>,
    invites_enabled: Option<bool>,
}

impl Instance {
    /// Get the domain name of this instance.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Get the title of this website.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get admin-defined description of this Mastodon site.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get a shorter description defined by the admin.
    pub fn short_description(&self) -> &str {
        &self.description
    }

    /// Get an email that may be contacted for any inquiries.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the version of Mastodon installed on this instance.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get primary langauges of this website and its staff.
    pub fn languages(&self) -> &Vec<String> {
        &self.languages
    }

    /// Get whether registrations are enabled.
    pub fn registrations(&self) -> bool {
        self.registrations
    }

    /// Get whether registrations require moderator approval.
    pub fn approval_required(&self) -> bool {
        self.approval_required
    }

    /// Get URLs of interest for clients apps.
    pub fn urls(&self) -> &Urls {
        &self.urls
    }

    /// Get statistics about how much information this instance contains.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Get banner image for this website.
    pub fn thumbnail(&self) -> Option<&Url> {
        self.thumbnail.as_ref()
    }

    // Get a user that can be contacted, as an alternative to email.
    pub fn contact_account(&self) -> Option<&Account> {
        self.contact_account.as_ref()
    }

    /// Get whether the invitation is enabled.(mastodon v3.1.4 or later)
    pub fn invites_enabled(&self) -> Option<bool> {
        self.invites_enabled
    }
}

/// URLs of interest for clients apps.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct Urls {
    streaming_api: Url,
}

impl Urls {
    /// Get websockets address for push streaming
    pub fn streaming_api(&self) -> &Url {
        &self.streaming_api
    }
}

/// Statistics about how much information the instance contains.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Deserialize)]
pub struct Stats {
    user_count: usize,
    status_count: usize,
    domain_count: usize,
}

impl Stats {
    /// Get users registered on this instance.
    pub fn user_count(&self) -> usize {
        self.user_count
    }

    /// Get statuses authored by users on this instance.
    pub fn status_count(&self) -> usize {
        self.status_count
    }

    /// Get domains federated with this instance.
    pub fn domain_count(&self) -> usize {
        self.domain_count
    }
}

pub type Peers = Vec<String>;
impl Entity for Peers {}
