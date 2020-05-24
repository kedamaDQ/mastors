pub trait Entity: std::marker::Sized + for<'de> serde::Deserialize<'de> {}

pub(crate) mod account;
pub(crate) mod activity;
pub(crate) mod application;
pub(crate) mod attachment;
pub(crate) mod card;
pub(crate) mod context;
pub(crate) mod emoji;
pub(crate) mod history;
pub(crate) mod instance;
pub(crate) mod mention;
pub(crate) mod notification;
pub(crate) mod privacy;
pub(crate) mod poll;
pub(crate) mod status;
pub(crate) mod tag;

pub use account::{ Account, Accounts };
pub use activity::{ Activity, Activities };
pub use application::Application;
pub use attachment::Attachment;
pub use card::Card;
pub use context::Context;
pub use emoji::{ Emoji, Emojis };
pub use history::History;
pub use instance::Instance;
pub use mention::Mention;
pub use notification::{ Notification, NotificationType };
pub use poll::Poll;
pub use privacy::{ Privacy, Visibility };
pub use status:: Status;
pub use tag::{ Tag, Trends };
