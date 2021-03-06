use serde::Deserialize;
use crate::{
    DateTime,
    Utc,
    Url,
};
use super::{
    Emoji,
    Entity,
    Privacy,
};

/// Represents a user of Mastodon and their associated profile.
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Account {
    // Base attributes
    #[mastors(identifier)]
    id: String, // cast from an integer, but not guaranteed to be a number

    username: String,
    acct: String,
    url: Url,

    // Display attributes
    display_name: String,
    note: String, // html
    avatar: Url,
    avatar_static: Url,
    header: Url,
    header_static: Url,
    locked: Option<bool>,
    emojis: Vec<Emoji>,
    discoverable: Option<bool>,

    // Statistical attributes
    created_at: DateTime<Utc>,
    statuses_count: usize,
    followers_count: usize,
    following_count: usize,

    // Optional attributes
    moved: Option<Box<Account>>,
    fields: Option<Vec<Field>>,
    bot: Option<bool>,
    source: Option<Source>,
}

impl Account {
    /// Get an ID of this account.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the username of this account, not including domain.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the Webfinger account URI. Equal to username for local users, or username@domain for remote users.
    pub fn acct(&self) -> &str {
        &self.acct
    }

    /// Get the location of the user's profile page as URL.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the profile's display name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Get the profile's bio / description as HTML.
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get the URL of an image icon that is shown next to statuses and in the profile.
    pub fn avatar(&self) -> &Url {
        &self.avatar
    }

    /// Get the URL of a static version of the avatar image icon. Equal to avatar if its value is a static image; different if avatar is an animated GIF.
    pub fn avatar_static(&self) -> &Url {
        &self.avatar_static
    }

    /// Get the URL of an image banner that is shown above the profile and in profile cards.
    pub fn header(&self) -> &Url {
        &self.header
    }

    /// Get the URL of a static version of the header. Equal to header if its value is a static image; different if header is an animated GIF.
    pub fn header_static(&self) -> &Url {
        &self.header_static
    }

    /// Get whether this account manually approves follow requests.
    pub fn locked(&self) -> Option<bool> {
        self.locked
    }

    /// Get custom emoji entities to be used when rendering the profile. If none, an empty array will be returned.
    pub fn emojis(&self) -> &Vec<Emoji> {
        &self.emojis
    }

    /// Get whether this account has opted into discovery features such as the profile directory.
    pub fn discoverable(&self) -> Option<bool> {
        self.discoverable
    }

    /// Get date time when this account was created.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get the number of statuses which are attached to this account.
    pub fn statuses_count(&self) -> usize {
        self.statuses_count
    }

    /// Get the reported followers of this profile.
    pub fn followers_count(&self) -> usize {
        self.followers_count
    }

    /// Get the reported follows of this profile.
    pub fn following_count(&self) -> usize {
        self.following_count
    }

    /// Get whether a profile is currently inactive and that its user has moved to a new account.
    pub fn moved(&self) -> Option<&Account> {
        self.moved.as_deref()
    }

    /// Get an additional metadata attached to a profile as name-value pairs.
    pub fn fields(&self) -> Option<&Vec<Field>> {
        self.fields.as_ref()
    }

    /// Get a presentational flag. Indicates that the account may perform automated actions, may not be monitored, or identifies as a robot.
    pub fn bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }

    /// Get an extra entity to be used with API methods to verify credentials and update credentials.
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Get whether this account is a local account.
    ///
    /// If true, this account is registered in the server that is you are connected to.
    pub fn is_local(&self) -> bool {
        self.acct == self.username
    }

    /// Get whether this account is a remote account.
    /// 
    /// If true, this account is registered on the server other than the server you are connected to.
    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }
}

/// Represents a profile field as a name-value pair with optional verification.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct Field {
    name: String,
    value: String, // html
    verified_at: Option<DateTime<Utc>>,
}

impl Field {
    /// Get the key of a given field's key-value pair.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the value associated with the `name` key.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get timestamp of when the server verified a URL value for a `rel="me”` link.
    pub fn verified_at(&self) -> Option<DateTime<Utc>> {
        self.verified_at
    }
}

/// Represents display or publishing preferences of user's own account. Returned as an additional entity when verifying and updated credentials, as an attribute of Account.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct Source {
    // Base attributes
    note: String,
    fields: Option<Vec<Field>>,

    // Nullable attributes
    privacy: Option<Privacy>,
    sensitive: Option<bool>,
    language: Option<String>, // ISO 639-1 language two-letter code
    follow_requests_count: usize,
}

impl Source {
    /// Get profile bio.
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get metadata about the account.
    pub fn fields(&self) -> Option<&Vec<Field>> {
        self.fields.as_ref()
    }

    /// Get default post privacy for authored statuses.
    pub fn privacy(&self) -> Option<Privacy> {
        self.privacy
    }

    /// Get whether new statuses should be marked sensitive by default.
    pub fn sensitive(&self) -> bool {
        self.sensitive.unwrap_or(false)
    }

    /// Get default language to use for authored statuses. (ISO 639-1)
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Get the number of pending follow requests.
    pub fn follow_requests_count(&self) -> usize {
        self.follow_requests_count
    }
}

/// Represents an array of [`Account`](./struct.Account.html)s.
pub type Accounts = Vec<Account>;
impl Entity for Accounts {}
