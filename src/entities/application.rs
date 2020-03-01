use serde::Deserialize;
use super::Entity;

/// Represents an application that interfaces with the REST API to access accounts or post statuses.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Deserialize)]
pub struct Application {
    // Required attributes
    name: String,

    // Optional attributes
    website: Option<String>,
    vapid_key: Option<String>,

    // Client attributes
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl Entity for Application {}

impl Application {
    /// Get the name of this application.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the website associated with this application.
    pub fn website(&self) -> &Option<String> {
        &self.website
    }

    /// Get the key used for Push Streaming API.
    pub fn vapid_key(&self) -> &Option<String> {
        &self.vapid_key
    }

    /// Get client ID key, to be used for obtaining OAuth tokens.
    pub fn client_id(&self) -> &Option<String> {
        &self.client_id
    }

    /// Get client secret key to be used for obtaining OAuth tokens.
    pub fn client_secret(&self) -> &Option<String> {
        &self.client_secret
    }
}
