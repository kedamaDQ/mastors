use serde::Deserialize;
use crate::utils::transform_string_to_usize;
use crate::utils::transform_string_to_i64;
use super::Entity;

/// Represents a weekly bucket of instance activity.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Deserialize, mastors_derive::Entity)]
pub struct Activity {
    #[serde(deserialize_with = "transform_string_to_i64")]
    week: i64,

    #[serde(deserialize_with = "transform_string_to_usize")]
    statuses: usize,

    #[serde(deserialize_with = "transform_string_to_usize")]
    logins: usize,

    #[serde(deserialize_with = "transform_string_to_usize")]
    registrations: usize,
}

impl Activity {
    /// Get midnight at the first day of the week.
    pub fn week(&self) -> i64 {
        self.week
    }

    /// Get statuses created since the week began.
    pub fn statuses(&self) -> usize {
        self.statuses
    }

    /// Get user logins since the week began.
    pub fn logins(&self) -> usize {
        self.logins
    }

    /// Get user registrations since the week began.
    pub fn registrations(&self) -> usize {
        self.registrations
    }
}

/// Represents an array of [`Activity`](./struct.Activity.html)s.
pub type Activities = Vec<Activity>;
impl Entity for Activities {}
