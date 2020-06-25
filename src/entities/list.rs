use serde::Deserialize;
use super::Entity;

/// Represents a list of some users that the authenticated user follows.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct List {
    id: String,
    title: String,
}

impl List {
    /// Get the internal database ID of this list.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Get the user defined title of this list.
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl Entity for List {}

/// Represents an array of [`List`](./struct.List.html)s.
pub type Lists = Vec<List>;
impl Entity for Lists {}
