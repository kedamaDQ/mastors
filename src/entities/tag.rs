use serde::Deserialize;
use super::Entity;

pub use crate::Url;
pub use super::History;

/// Represents a hashtag used within the content of a status.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct Tag {
    name: String,
    url: crate::Url,
    history: Option<Vec<History>>,
}

impl Tag {
    /// Get the value of the hashtag. This method will return string without "#" sign.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a link to the hashtag on the instance.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get usage statistics for givn days.
    pub fn history(&self) -> &Option<Vec<History>> {
        &self.history
    }
}

impl Entity for Tag {}

/// Represents an array of [`Tag`](./struct.Tag.html)s.
pub type Trends = Vec<Tag>;
impl Entity for Trends {}
