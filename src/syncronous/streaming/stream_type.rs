
/// Represents a streaming type.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone)]
pub enum StreamType {
    /// Represents stream of events that are relevant to the authorized user, i.e. home timeline and notifications.
    User,

    /// Represents stream of all public statuses.
    Public,

    /// Represents stream of all local statuses.
    PublicLocal,

    /// Represents stream of all public statuses without local statuses. (mastodon v3.1.4 or later)
    PublicRemote,

    /// Represents stream of all public statuses for a particular hashtag.
    Hashtag(String),

    /// Represents stream of all local statuses for a particular hashtag.
    HashtagLocal(String),

    /// Represents stream of all statuses for a list.
    List(String),

    /// Represents stream of all direct messages.
    Direct,
}

use std::fmt;

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ENDPOINT: &str = "/api/v1/streaming";

        match self {
            StreamType::User => write!(f, "{}/user", ENDPOINT),
            StreamType::Public => write!(f, "{}/public", ENDPOINT),
            StreamType::PublicLocal => write!(f, "{}/public/local", ENDPOINT),
            StreamType::PublicRemote => write!(f, "{}/public/remote", ENDPOINT),
            StreamType::Hashtag(tag) => write!(f, "{}/hashtag?tag={}", ENDPOINT, tag),
            StreamType::HashtagLocal(tag) => write!(f, "{}/hashtag/local?tag={}", ENDPOINT, tag),
            StreamType::List(id) => write!(f, "{}/list?list={}", ENDPOINT, id),
            StreamType::Direct => write!(f, "{}/direct", ENDPOINT),
        }
    }
}

