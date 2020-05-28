use crate::{
    Result,
    entities::{
        Notification,
        Status,
    },
};

pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {}

/// Represent the event types of the server-sent events of the Mastodon.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EventType {
    /// A new status has appeared in the timeline.
    /// 
    /// This event type has `Status`.
    Update(Box<Status>),

    /// A new notification has appeared.
    /// 
    /// This event type has `Notification`.
    Notification(Box<Notification>),

    /// A status as been deleted.
    /// 
    /// This event type has ID of the deleted status as `String`.
    Delete(String),

    /// Keyword filters have been changed.
    /// 
    /// This event type has no data.
    FiltersChanged,

    /// This event type contains received raw data as `String`.
    Unknown(String),
}
