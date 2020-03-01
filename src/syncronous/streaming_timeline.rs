use crate::{
    Result,
    entities::{
        Notification,
        Status,
    },
};

pub trait StreamingTimeline: Iterator<Item = Result<EventType>> {}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EventType {
    Update(Box<Status>),
    Notification(Box<Notification>),
    Delete(String),
    FiltersChanged,
    Unknown(String),
}
