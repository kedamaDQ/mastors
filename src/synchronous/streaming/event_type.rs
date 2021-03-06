use crate::entities::{
	Notification,
	Status,
};

/// Represent the event types of the streaming timeline.
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
