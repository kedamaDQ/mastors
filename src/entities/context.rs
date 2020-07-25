use serde::Deserialize;
use super::{
    Status,
};

/// Represents the tree around the given status.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, mastors_derive::Entity)]
pub struct Context {
    // Required attributes
    ancestors: Vec<Status>,
    descendants: Vec<Status>,
}

impl Context {
    /// Get status that are ancestors of the given status.
    pub fn ancestors(&self) -> &Vec<Status> {
        &self.ancestors
    }

    /// Get status that are descendants of the given status.
    pub fn descendants(&self) -> &Vec<Status> {
        &self.descendants
    }
}
