use serde::Deserialize;
use crate::Url;

/// Represents a rich preview card that is generated using OpenGraph tags from a URL.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize, mastors_derive::Entity)]
pub struct Card {
    // Required attributes
    url: Url,
    title: String,
    description: String,
    r#type: CardType,

    //Optional attributes
    author_name: Option<String>,
    author_url: Option<Url>,
    provider_name: Option<String>,
    provider_uri: Option<Url>,
    html: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    image: Option<Url>,
    embed_url: Option<Url>,
}

impl Card {
    /// Get the location of linked resource.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the title of linked.resource.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the description of preview.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the type of the preview card as `CardType`.
    pub fn r#type(&self) -> CardType {
        self.r#type
    }

    /// Get the author of the original resource.
    pub fn author_name(&self) -> Option<&str> {
        self.author_name.as_deref()
    }

    /// Get a link to the author of the original resource.
    pub fn author_url(&self) -> Option<&Url> {
        self.author_url.as_ref()
    }

    /// Get the provider of the original resource.
    pub fn provider_name(&self) -> Option<&str> {
        self.provider_name.as_deref()
    }

    /// Get a link to the provider of the original resource.
    pub fn provider_uri(&self) -> Option<&Url> {
        self.provider_uri.as_ref()
    }

    /// Get HTML to be used for generating the preview card.
    pub fn html(&self) -> Option<&str> {
        self.html.as_deref()
    }

    /// Get width of preview, in pixels.
    pub fn width(&self) -> Option<u32> {
        self.width
    }

    /// Get height of preview, in pixels.
    pub fn height(&self) -> Option<u32> {
        self.height
    }

    /// Get the location of preview thumbnail.
    pub fn image(&self) -> Option<&Url> {
        self.image.as_ref()
    }

    /// Get the location of photo embeds which is used instead of custom html.
    pub fn embed_url(&self) -> Option<&Url> {
        self.embed_url.as_ref()
    }

    /// Get the type of the preview card as `CardType`.
    /// 
    /// This method is an alias of `r#type()`.
    pub fn card_type(&self) -> CardType {
        self.r#type()
    }

    /// Get whether this card is a `link`.
    pub fn is_link(&self) -> bool {
        self.r#type == CardType::Link
    }

    /// Get whether this card is a `photo`.
    pub fn is_photo(&self) -> bool {
        self.r#type == CardType::Photo
    }

    /// Get whether this card is a `video`.
    pub fn is_video(&self) -> bool {
        self.r#type == CardType::Video
    }

    /// Get whether this card is a `rich`.
    pub fn is_rich(&self) -> bool {
        self.r#type == CardType::Rich
    }
}

/// The type of the preview card.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum CardType {
    /// Link OEmbed
    Link,

    /// Photo OEmbed
    Photo,

    /// Vide OEmbed
    Video,

    /// iframe OEmbed. Not currently accepted, so won't show up in practice.
    Rich,
}

use std::{ fmt, str::FromStr };

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Link => write!(f, "link"),
            CardType::Photo => write!(f, "photo"),
            CardType::Video => write!(f, "video"),
            CardType::Rich => write!(f, "rich"),
        }
    }
}

impl FromStr for CardType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "link" => Ok(CardType::Link),
            "photo" => Ok(CardType::Photo),
            "video" => Ok(CardType::Video),
            "rich" => Ok(CardType::Rich),
            _ => Err(crate::Error::ParseCardTypeError(s.to_owned())),
        }
    }
}

use serde::de;

impl<'de> de::Deserialize<'de> for CardType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
		let s = String::deserialize(deserializer)?;
		match CardType::from_str(s.as_str()) {
			Ok(r) => Ok(r),
			Err(e) => Err(de::Error::custom(e)),
		}
    }
}
