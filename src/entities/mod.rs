//! This module provides entities that are the response of the API methods.
//! 
//! e.g. [`mastors::entities::Instance`](./struct.Instance.html) is entity returned by API method [`mastors::api::v1::instance::get()`](../api/v1/instance/fn.get.html).
pub trait Entity: std::fmt::Debug + std::marker::Sized + for<'de> serde::Deserialize<'de> {}

pub(crate) mod account;
pub(crate) mod activity;
pub(crate) mod application;
pub(crate) mod attachment;
pub(crate) mod card;
pub(crate) mod context;
pub(crate) mod emoji;
pub(crate) mod history;
pub(crate) mod identity_proof;
pub(crate) mod instance;
pub(crate) mod list;
pub(crate) mod markers;
pub(crate) mod mention;
pub(crate) mod notification;
pub(crate) mod page_navigation;
pub(crate) mod privacy;
pub(crate) mod poll;
pub(crate) mod relationship;
pub(crate) mod scheduled_status;
pub(crate) mod status;
pub(crate) mod tag;

pub use account::{ Account, Accounts };
pub use activity::{ Activity, Activities };
pub use application::Application;
pub use attachment::Attachment;
pub use card::Card;
pub use context::Context;
pub use emoji::{ Emoji, Emojis };
pub use history::History;
pub use identity_proof::{ IdentityProof, IdentityProofs };
pub use instance::Instance;
pub use list::{ List, Lists };
pub use markers::{ Marker, Markers };
pub use mention::Mention;
pub use notification::{ Notification, Notifications, NotificationType };
pub use page_navigation::PageNavigation;
pub use poll::Poll;
pub use privacy::{ Privacy, Visibility };
pub use relationship::{ Relationship, Relationships };
pub use scheduled_status::{ DeletedScheduledStatus, Params, ScheduledStatus, ScheduledStatuses, ScheduledPoll };
pub use status::{ Status, Statuses };
pub use tag::{ Tag, Trends };

/// Represents a no body response.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, serde::Deserialize, mastors_derive::Entity)]
pub struct Nothing {}
