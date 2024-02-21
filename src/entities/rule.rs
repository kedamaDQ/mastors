/// this module provides features related to server rule
use serde::Deserialize;
use super::Entity;

/// Represents a server rule
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Rule {
    #[mastors(identifier)]
    id: String,
    text: String,
}

impl Rule {
    /// Get an ID of the report
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Represents an array of [`Rule`]es
pub type Rules = Vec<Rule>;
impl Entity for Rules {}