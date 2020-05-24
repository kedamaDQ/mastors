use serde::Deserialize;
use crate::utils::transform_str_to_enum;
use super::Entity;

pub use crate::{
    DateTime,
    Utc,
    Url,
};
pub use super::{
    Account,
    Application,
    Attachment,
    Card,
    Emoji,
    Mention,
    Poll,
    Tag,
    Visibility,
};

/// Represents a status posted by an account.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct Status {
    // Base attributes
    id: String,
    uri: Url,
    created_at: DateTime<Utc>,
    account: Box<Account>,
    content: Option<String>,
    #[serde(deserialize_with = "transform_str_to_enum")]
    visibility: Visibility,
    sensitive: bool,
    spoiler_text: String,
    media_attachments: Vec<Attachment>,
    application: Application,

    // Rendering attributes
    mentions: Vec<Mention>,
    tags: Vec<Tag>,
    emojis: Vec<Emoji>,

    // Information attributes
    reblogs_count: u64,
    favourites_count: u64,
    replies_count: u64,

    // Nullable attributes
    url: Option<Url>,
    in_reply_to_id: Option<String>,
    in_reply_to_account_id: Option<String>,
    reblog: Option<Box<Status>>,
    poll: Option<Poll>,
    card: Option<Card>,
    language: Option<String>,
    text: Option<String>,

    // Authorized user attributes
    favourited: Option<bool>,
    reblogged: Option<bool>,
    muted: Option<bool>,
    bookmarked: Option<bool>,
    pinned: Option<bool>,
}

impl Status {
    /// Get the ID of this status in the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the URI of this status used for federation.
    pub fn uri(&self) -> &Url {
        &self.uri
    }

    /// Get the date time when this status was created.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Get the account that authored this status.
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Get the status content as HTML.
    /// 
    /// This method will returns `None` when this status returned by status deletion API.
    /// In this case, you can use the `text()` method instead to get the non-HTML content.
    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    /// Get the `Visibility` of this status.
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Get whether this status marked as sensitive content.
    pub fn sensitive(&self) -> bool {
        self.sensitive
    }

    /// Get the subject or summary line, below which status content is collapsed until expanded.
    pub fn spoiler_text(&self) -> &str {
        &self.spoiler_text
    }

    /// Get medias that is attached to this status.
    /// 
    /// This method returns one or more `Attachment` only if this status is returned by the status deletion API and this status has `Attachment`.
    pub fn media_attachments(&self) -> &Vec<Attachment> {
        &self.media_attachments
    }

    /// Get the application used to post this status.
    pub fn application(&self) -> &Application {
        &self.application
    }

    /// Get mentions of users within this status content.
    pub fn mentions(&self) -> &Vec<Mention> {
        &self.mentions
    }

    /// Get hashtags used within this status content.
    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    /// Get custom emojis used within this status content.
    pub fn emojis(&self) -> &Vec<Emoji> {
        &self.emojis
    }

    /// Get a number of boosts this status received.
    pub fn reblogs_count(&self) -> u64 {
        self.reblogs_count
    }

    /// Get a number of favourites this status received.
    pub fn favourites_count(&self) -> u64 {
        self.favourites_count
    }

    /// Get a number of favourites this status received.
    pub fn replies_count(&self) -> u64 {
        self.replies_count
    }

    /// Get a link to this status's HTML representation.
    pub fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }

    /// Get an ID of the status being replied.
    pub fn in_reply_to_id(&self) -> Option<&String> {
        self.in_reply_to_id.as_ref()
    }

    /// Get an ID of the account being replied to.
    pub fn in_reply_to_account_id(&self) -> Option<&String> {
        self.in_reply_to_account_id.as_ref()
    }

    /// Get the status being reblogged.
    pub fn reblog(&self) -> Option<&Box<Status>> {
        self.reblog.as_ref()
    }

    /// Get the poll attached this status.
    pub fn poll(&self) -> Option<&Poll> {
        self.poll.as_ref()
    }

    /// Get the preview card for links included within this status content.
    pub fn card(&self) -> Option<&Card> {
        self.card.as_ref()
    }

    /// Get primary language of this status which is compliant to ISO 639-1.
    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Get plain-text source of this status. 
    /// 
    /// This method will return non-HTML content instead of `content()` only if this status returned by status deletion API, so the user may redraft from the source text without the client having to reverse-engineer the original text from the HTML content.
    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    /// Get whether authorized user has favourited this status.
    pub fn favourited(&self) -> bool {
        self.favourited.unwrap_or(false)
    }

    /// Get whether authorized user has reblogged this status.
    pub fn reblogged(&self) -> bool {
        self.reblogged.unwrap_or(false)
    }

    /// Get whether authorized user has muted this status.
    pub fn muted(&self) -> bool {
        self.muted.unwrap_or(false)
    }

    /// Get whether authorized user has bookmarkedn this status.
    pub fn bookmarked(&self) -> bool {
        self.bookmarked.unwrap_or(false)
    }

    /// Get whether authorized user has pinned this status.
    pub fn pinned(&self) -> bool {
        self.pinned.unwrap_or(false)
    }
}

impl Entity for Status {}
