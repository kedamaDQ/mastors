use serde::Deserialize;
use crate::{
    DateTime,
    Error,
    Utc,
};
use super::{
    Account,
    Entity,
    Status,
};

/// Represents a receive notification for activity on your account or statuses.
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Notification {
    // Required attributes
    #[mastors(identifier)]
    id: String,
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
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get the account that performed the action that generated the notification.
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Get status that was the object of the notification, e.g. in mentions, reblogs, favourites, or polls.
    pub fn status(&self) -> Option<&Status> {
        self.status.as_deref()
    }

    /// Get the type of event that resulted in the notification.
    /// 
    /// This method is an alias of `r#type()`.
    pub fn notification_type(&self) -> NotificationType {
        self.r#type()
    }

    /// Get whether this is `follow` notification.
    pub fn is_follow(&self) -> bool {
        self.r#type == NotificationType::Follow
    }

    /// Get whether this is `mention` notification.
    pub fn is_mention(&self) -> bool {
        self.r#type == NotificationType::Mention
    }

    /// Get whether this is `reblog` notification.
    pub fn is_reblog(&self) -> bool {
        self.r#type == NotificationType::Reblog
    }

    /// Get whether this is `favourite` notification.
    pub fn is_favourite(&self) -> bool {
        self.r#type == NotificationType::Favourite
    }

    /// Get whether this is `poll` notification.
    pub fn is_poll(&self) -> bool {
        self.r#type == NotificationType::Poll
    }

    /// Get whether this is `follow_request` notification.
    pub fn is_follow_request(&self) -> bool {
        self.r#type == NotificationType::FollowRequest
    }

    pub fn is_update(&self) -> bool {
        self.r#type == NotificationType::Update
    }

    pub fn is_admin_sign_up(&self) -> bool {
        self.r#type == NotificationType::AdminSignUp
    }

    pub fn is_admin_report(&self) -> bool {
        self.r#type == NotificationType::AdminReport
    }
}

/// Represents an Array of [`Notification`](/entities/struct.Notification.html).
pub type Notifications = Vec<Notification>;
impl Entity for Notifications {}

use std::str::FromStr;

/// Represents a type of event that resulted in the notification.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

    /// Someone requested to authorize to follow you.
    FollowRequest,

    /// Someone you enabled notifications for has posted a status
    Status,

    /// A status you boosted with has been edited
    Update,

    /// Someone signed up (optionally sent to admins)
    AdminSignUp,

    /// A new report has been filed
    AdminReport
}

use std::fmt;

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reblog => write!(f, "reblog"),
            NotificationType::Favourite => write!(f, "favourite"),
            NotificationType::Poll => write!(f, "poll"),
            NotificationType::FollowRequest => write!(f, "follow_request"),
            NotificationType::Status => write!(f, "status"),
            NotificationType::Update => write!(f, "update"),
            NotificationType::AdminSignUp => write!(f, "admin.sign_up"),
            NotificationType::AdminReport => write!(f, "admin.report"),
        }
    }
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
            "follow_request" => Ok(NotificationType::FollowRequest),
            "status" => Ok(NotificationType::Status),
            "update" => Ok(NotificationType::Update),
            "admin.sign_up" => Ok(NotificationType::AdminSignUp),
            "admin.report" => Ok(NotificationType::AdminReport),
            _ => Err(Error::ParseNotificationTypeError(s.to_owned()))
        }
    }
}

use serde::{ ser, de };

impl ser::Serialize for NotificationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for NotificationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
		let s = String::deserialize(deserializer)?;
		match NotificationType::from_str(s.as_str()) {
			Ok(r) => Ok(r),
			Err(e) => Err(de::Error::custom(e)),
		}
    }
}
