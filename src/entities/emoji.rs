use serde::Deserialize;
use crate::Url;
use super::Entity;


/// Represents a custom emoji.
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Emoji {
    #[mastors(identifier)]
    shortcode: String,
    url: Url,
    static_url: Url,
    visible_in_picker: bool,
    category: Option<String>,
}

impl Emoji {
    /// Get the name of this custom emoji.
    pub fn shortcode(&self) -> &str {
        &self.shortcode
    }

    /// Get a link to this custom emoji.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get a link to a static copy of this custom emoji.
    pub fn static_url(&self) -> &Url {
        &self.static_url
    }

    /// Get whether this custom emoji should be visible in the picker or unlisted.
    pub fn visible_in_picker(&self) -> &bool {
        &self.visible_in_picker
    }

    /// Get the category of this custom emoji used for sorting custom emoji in the picker.
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }
}

/// Represents an array of [`Emoji`](./struct.Emoji.html)s.
pub type Emojis = Vec<Emoji>;
impl Entity for Emojis {}
