use async_trait::async_trait;
use serde::Serialize;
use crate::{
    Connection,
    Error,
    Result,
    entities::Attachment,
    methods::{
        Method,
        Upload,
        FileForm,
    }
};

pub fn post(conn: &Connection, filename: impl Into<String>) -> PostMedia {
    PostMedia {
        conn,
        filename: filename.into(),
        description: None,
        focus: None,
        focus_str: None,
    }
}

#[derive(Debug, Serialize)]
pub struct PostMedia<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    filename: String,
    description: Option<String>,
    focus: Option<Focus>,
    focus_str: Option<String>,
}

impl<'a> PostMedia<'a> {
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn focus(mut self, focus: Focus) -> Self {
        self.focus_str = Some(format!("{}", focus));
        self.focus = Some(focus);
        self
    }
}

#[async_trait]
impl<'a> Method<'a, Attachment> for PostMedia<'a> {
    const ENDPOINT: &'a str = "/api/v1/media";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization_code(&self) -> Option<&str>{
        Some(self.conn.access_token())
    }

    async fn send(&'a self) -> Result<Attachment> {
        Ok(self.post_with_media().await?)
    }
}

#[async_trait]
impl<'a> Upload<'a, Attachment> for PostMedia<'a> {
    fn textforms(&self) -> Vec<(&str, &str)> {
        let mut forms: Vec<(&str, &str)> = Vec::new();

        if let Some(description) = &self.description {
            forms.push(("description", description));
        }

        if let Some(focus_str) = &self.focus_str {
            forms.push(("focus", focus_str));
        }

        forms
    }

    fn fileform(&self) -> FileForm<'_> {
        FileForm {
            filename: &self.filename,
            formname: "file",
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize)]
pub struct Focus {
    x: f64,
    y: f64,
}

impl Focus {
    const MIN_FOCUS_VALUE: f64 = -1.0;
    const MAX_FOCUS_VALUE: f64 = 1.0;

    pub fn new(x: f64, y: f64) -> Result<Self> {

        let range = Self::MIN_FOCUS_VALUE ..= Self::MAX_FOCUS_VALUE;

        if range.contains(&x) && range.contains(&y) {
            Ok(Self { x, y })
        } else {
            Err(
                Error::InvalidFocalPointError(x, y, Self::MIN_FOCUS_VALUE, Self::MAX_FOCUS_VALUE)
            )
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

use std::fmt;

impl fmt::Display for Focus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
