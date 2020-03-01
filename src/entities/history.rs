use serde::Deserialize;
use crate::utils::transform_string_to_u64;
use super::Entity;

/// Represents daily usage history of a hashtag.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Copy, Deserialize)]
pub struct History {
    #[serde(deserialize_with = "transform_string_to_u64")]
    day: u64,

    #[serde(deserialize_with = "transform_string_to_u64")]
    uses: u64,

    #[serde(deserialize_with = "transform_string_to_u64")]
    accounts: u64,
}

impl History {
    /// Get UNIX timestamp on midnight of the given day.
    pub fn day(&self) -> u64 {
        self.day
    }

    /// Get the counted usage of the tag within that day.
    pub fn uses(&self) -> u64 {
        self.uses
    }

    /// Get the total of accounts using the tag within that day.
    pub fn accounts(&self) -> u64 {
        self.accounts
    }
}

impl Entity for History {}
