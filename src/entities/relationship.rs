//! Represents relationship between accounts.
use serde::Deserialize;
use super::Entity;

#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Relationship {
    #[mastors(identifier)]
    id: String,

    following: bool,
    requested: bool,
    endorsed: bool,
    followed_by: bool,
    muting: bool,
    muting_notifications: bool,
    showing_reblogs: bool,
    blocking: bool,
    domain_blocking: bool,
    blocked_by: bool,
}

impl Relationship {
    /// Get an ID of related account.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Get whether you are following this account.
    pub fn following(&self) -> bool {
        self.following
    }

    /// Get whether you have a follow request fot this account.
    pub fn requested(&self) -> bool {
        self.requested
    }

    /// Get whether you have endorsed this account. 
    pub fn endorsed(&self) -> bool {
        self.endorsed
    }

    /// Get whether you are followed by this account.
    pub fn followed_by(&self) -> bool {
        self.followed_by
    }

    /// Get whether you are muting this account.
    pub fn muting(&self) -> bool {
        self.muting
    }

    /// Get whether you are muting notifications related to this account.
    pub fn muting_notifications(&self) -> bool {
        self.muting_notifications
    }

    /// Get whether you are showing reblogs by this account on your timeline.
    pub fn showing_reblogs(&self) -> bool {
        self.showing_reblogs
    }

    /// Get whether you are blocking this account.
    pub fn blocking(&self) -> bool {
        self.blocking
    }

    /// Get whether you are blocking domain that is this account belongs.
    pub fn domain_blocking(&self) -> bool {
        self.domain_blocking
    }

    /// Get whether you are blocked by this account.
    pub fn blocked_by(&self) -> bool {
        self.blocked_by
    }
}

/// Represents an array of [`Relationship`](./struct.Relationship.html)s.
pub type Relationships = Vec<Relationship>;
impl Entity for Relationships {}
