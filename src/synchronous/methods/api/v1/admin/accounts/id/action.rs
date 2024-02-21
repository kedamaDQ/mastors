/// this module provides features related to moderate account for moderators
use serde::Serialize;
use crate::{
    Connection,
    Error,
    Method,
    entities::{
        Dummy,
    },
};

/// Create POST request for /api/v1/admin/accounts/:id/action
pub fn post(conn: &Connection, id: impl Into<String>, r#type: ActionType) -> PostAction {
    PostAction {
        conn,
        id: id.into(),
        r#type,
        authorized: true,
        report_id: None,
        warning_preset_id: None,
        text: None,
        send_email_notification: false,
    }
}

/// POST request for /api/v1/admin/accounts/:id/action
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Dummy, "/api/v1/admin/accounts/_PATH_PARAM_/action")]
pub struct PostAction<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    r#type: ActionType,
    report_id: Option<String>,
    warning_preset_id: Option<String>,
    text: Option<String>,
    send_email_notification: bool,
}

impl<'a> PostAction<'a> {
    /// Set report ID if the action related to report to close
    pub fn report_id(mut self, report_id: impl Into<String>) -> Self {
        let report_id = report_id.into();
        self.report_id = if report_id.trim().is_empty() {
            None
        } else {
            Some(report_id)
        };
        self
    }

    /// ??? (nothing by default)
    pub fn warning_preset_id(mut self, warning_preset_id: impl Into<String>) -> Self {
        let warning_preset_id = warning_preset_id.into();
        self.warning_preset_id = if warning_preset_id.trim().is_empty() {
            None
        } else {
            Some(warning_preset_id)
        };
        self
    }

    /// ??? (nothing by default)
    pub fn text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.text = if text.trim().is_empty() {
            None
        } else {
            Some(text)
        };
        self
    }

    /// Set whether to send to user an email (false by defaul)
    pub fn send_email_notification(mut self, send_email_notification: bool) -> Self {
        self.send_email_notification = send_email_notification;
        self
    }
}

impl<'a> Method<'a, Dummy> for PostAction<'a> {}

/// Represents action types e.g. Sensitive, Silence, Suspend
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ActionType {
    None,
    Sensitive,
    Disable,
    Silence,
    Suspend,
}

use std::fmt;

impl fmt::Display for ActionType {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::None => write!(f, "none"),
            ActionType::Sensitive => write!(f, "sensitive"),
            ActionType::Disable => write!(f, "disable"),
            ActionType::Silence => write!(f, "silence"),
            ActionType::Suspend => write!(f, "suspend"),
        }
    }
}

use std::str::FromStr;

impl FromStr for ActionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(ActionType::None),
            "sensitive" => Ok(ActionType::Sensitive),
            "disable" => Ok(ActionType::Disable),
            "silence" => Ok(ActionType::Silence),
            "suspend" => Ok(ActionType::Suspend),
            _ => Err(Error::ParseActionTypeError(s.to_owned())),
        }
    }
}

use serde::{ ser, de };

impl ser::Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
		let s = String::deserialize(deserializer)?;
		match ActionType::from_str(s.as_str()) {
			Ok(r) => Ok(r),
			Err(e) => Err(de::Error::custom(e)),
		}
    }
}
