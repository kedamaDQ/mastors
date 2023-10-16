use chrono::{ DateTime, Utc };
use err_derive::Error;
use serde::Deserialize;

use crate::Url;

/// A `Result` alias where the `Err` case is `mastors::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that may occur when communicating with the Mastodon.
#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Environment file '{}' not found: {}", path, source)]
    EnvNotFoundError {
        #[error(source)]
        source: dotenv::Error,
        path: String,
    },

    #[error(display = "Environment variable error: '{}', {}", env_var, source)]
    EnvVarError {
        #[error(source)]
        source: std::env::VarError,
        env_var: &'static str
    },

    #[error(display = "'{}' is not a valid number: {}", env_var, source)]
    ParseEnvVarError {
        #[error(source)]
        source: std::num::ParseIntError,
        env_var: &'static str,
    },

    #[error(display = "file io error")]
    FileIoError(
        #[error(source, from)]
        std::io::Error,
    ),

    #[error(display = "'{}' is not a file", _0)]
    NotFileError(String),

    #[error(display = "Size of '{}' is zero", _0)]
    BlankFileError(String),

    #[error(display = "Parse URL error: {}", source)]
    ParseUrlError{
        #[error(source, from)]
        source: url::ParseError,
    },

    #[error(display = "'{}' is not a valid Privacy string", _0)]
    ParsePrivacyError(String),

    #[error(display = "'{}' is not a valid AttachmentType string", _0)]
    ParseAttachmentTypeError(String),

    #[error(display = "'{}' is not a valid NotificationType string", _0)]
    ParseNotificationTypeError(String),

    #[error(display = "'{}' is not a valid CardType string", _0)]
    ParseCardTypeError(String),

    #[error(display = "'{}' is not a valid Scope string", _0)]
    ParseScopeError(String),

    #[error(display = "'{}' is not ISO639-1 compliant", _0)]
    ParseIso639_1Error(String),

    #[error(display = "HTTP client error: {}", _0)]
    HttpClientError(
        #[error(source, no_from)]
        reqwest::Error,
    ),

    #[error(display = "HTTP request error: {}", _0)]
    HttpRequestError(
        #[error(source, from)]
        reqwest::Error,
    ),

    #[error(display = "HTTP client error: {} ({}) {}", _0, _1, _2)]
    HttpClientStatusError(Url, u16, Box<ReceivedMessage>),

    #[error(display = "HTTP server error: {} ({})", _0, _1)]
    HttpServerStatusError(Url, u16),

    #[error(display = "HTTP unknown error: {} ({}) Parhaps, this is a mastors bug", _0, _1)]
    HttpUnexpectedStatusError(Url, u16),

    #[error(display = "{} is not a valid HTTP header value", _0)]
    InvalidHeaderValueError(
        #[error(source, from)]
        reqwest::header::InvalidHeaderValue,
    ),

    #[error(display = "HTTP header value error: {}", _0)]
    HeaderValueToStrError(
        #[error(source, from)]
        reqwest::header::ToStrError,
    ),

    #[error(display = "Server-sent events error")]
    SseStreamError(
        #[error(source, from)]
        eventsource::reqwest::Error,
    ),

    /*
    #[error(display = "WebSocket error")]
    WebSocketError(
        #[error(source, from)]
        async_tungstenite::tungstenite::Error,
    ),
    */

    #[error(display = "Failed to deserialize entity, perhaps, this is a bug of mastors: {}", _0)]
    DeserializeJsonError(
        #[error(source, from)]
        serde_json::error::Error,
    ),

    #[error(display = "Received Unknown event type '{}'", _0)]
    UnknownEventTypeError(String),

    #[error(display = "Status requires status content text")]
    InvalidStatusError,

    #[error(display = "Too many characters in a status (max: {}, got: {})", _1, _0)]
    TooManyCharactersError(usize, usize),

    #[error(display = "Attachment media is nothing")]
    NoAttachmentMediaError,

    #[error(display = "Too many media attachments: max: {}, got: {}", _1, _0)]
    TooManyAttachmentMediasError(usize, usize),

    #[error(display = "Attachment media is duplicate")]
    DuplicateMediaError,

    #[error(display = "Focal point value allows a number between {} and {} but got (x: {}, y: {})", _2, _3, _0, _1)]
    InvalidFocalPointError(f64, f64, f64, f64),

    #[error(display = "The poll requires least 2 options")]
    TooLittlePollOptionsError,

    #[error(display = "Too many poll options: max: {}, got: {}", _1, _0)]
    TooManyPollOptionsError(usize, usize),

    #[error(display = "Poll option is duplicate")]
    DuplicatePollOptionError,

    #[error(display = "{} is a past date time", _0)]
    PastDateTimeError(DateTime<Utc>),

    #[error(display = "Schedule is too close: now: {}, scheduled: {}", _0, _1)]
    ScheduleTooCloseError(DateTime<Utc>, DateTime<Utc>),

    #[error(display = "Voted option is duplicate")]
    DuplicateVoteOptionError,

    #[error(display = "Account IDs are duplicate")]
    DuplicateAccountIdError,

    #[error(display = "Account ID is nothing")]
    NoAccountIdError,

    #[error(display = "No timeline specified")]
    NoTimelineError,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Deserialize)]
pub struct ReceivedMessage {
    error: Option<String>,
    error_description: Option<String>,
}

use std::fmt;

impl fmt::Display for ReceivedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{{ error: {}, error_description: {} }}",
            self.error.as_ref().unwrap_or(&"None".to_owned()),
            self.error_description.as_ref().unwrap_or(&"None".to_owned())
        )
    }
}
