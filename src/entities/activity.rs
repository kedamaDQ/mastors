use serde::Deserialize;
use crate::utils::transform_string_to_u64;
use super::Entity;

/// Represents a weekly bucket of instance activity.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Copy, Deserialize)]
pub struct Activity {
    #[serde(deserialize_with = "transform_string_to_u64")]
    week: u64,

    #[serde(deserialize_with = "transform_string_to_u64")]
    statuses: u64,

    #[serde(deserialize_with = "transform_string_to_u64")]
    logins: u64,

    #[serde(deserialize_with = "transform_string_to_u64")]
    registrations: u64,
}

impl Activity {
    /// Get midnight at the first day of the week.
    pub fn week(&self) -> u64 {
        self.week
    }

    /// Get statuses created since the week began.
    pub fn statuses(&self) -> u64 {
        self.statuses
    }

    /// Get user logins since the week began.
    pub fn logins(&self) -> u64 {
        self.logins
    }

    /// Get user registrations since the week began.
    pub fn registrations(&self) -> u64 {
        self.registrations
    }
}

impl Entity for Activity {}

/// Represents an array of [`Activity`](./struct.Activity.html)s.
pub type Activities = Vec<Activity>;
impl Entity for Activities {}
