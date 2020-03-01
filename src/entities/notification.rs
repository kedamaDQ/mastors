use serde::Deserialize;
use crate::utils::transform_str_to_enum;
use super::Entity;

pub use crate::{
    DateTime,
    Error,
    Utc,
};
pub use super::{
    Account,
    Status,
};

/// Represents a receive notification for activity on your account or statuses.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct Notification {
    // Required attributes
    id: String,

    #[serde(deserialize_with = "transform_str_to_enum")]
    r#type: NotificationType,

    created_at: DateTime<Utc>,
    account: Box<Account>,

    // Optional attributes
    status: Option<Box<Status>>,
}

impl Notification {
    /// Get the id of the notification in the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the type of event that resulted in the notification.
    pub fn r#type(&self) -> NotificationType {
        self.r#type
    }

    /// Get the timestamp of the notification.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Get the account that performed the action that generated the notification.
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Get status that was the object of the notification, e.g. in mentions, reblogs, favourites, or polls.
    pub fn status(&self) -> &Option<Box<Status>> {
        &self.status
    }
}

impl Entity for Notification {}

use std::str::FromStr;

/// Represents a type of event that resulted in the notification.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Deserialize)]
pub enum NotificationType {
    /// Someone followed you.
    Follow,

    /// Someone mentioned you in their status.
    Mention,

    /// Someone boosted one of your status.
    Reblog,

    /// Someone favourited one of your statuses.
    Favourite,

    /// A poll you have voted in or created has ended.
    Poll,
}

impl FromStr for NotificationType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "follow" => Ok(NotificationType::Follow),
            "mention" => Ok(NotificationType::Mention),
            "reblog" => Ok(NotificationType::Reblog),
            "favourite" => Ok(NotificationType::Favourite),
            "poll" => Ok(NotificationType::Poll),
            _ => Err(Error::ParseNotificationTypeError(s.to_owned()))
        }
    }
}
