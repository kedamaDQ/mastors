use serde::Serialize;
use crate::{
    Connection,
    Error,
    Result,
    entities::Attachment,
    methods::{
        Method,
        MethodInternal,
        UploadInternal,
        FileFormInternal,
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
    pub fn description(&mut self, description: impl Into<String>) -> &Self {
        self.description = Some(description.into());
        self
    }

    pub fn focus(&mut self, x: f64, y: f64) -> &Self {
        let focus = Focus::new(x, y);
        self.focus_str = Some(focus.to_string());
        self.focus = Some(focus);
        self
    }
}

impl<'a> Method<'a, Attachment> for PostMedia<'a> {
    fn send(&'a self) -> Result<Attachment> {
        if let Some(focus) = &self.focus {
            focus.validate()?;
        }
        Ok(self.post_with_media()?)
    }
}

impl<'a> MethodInternal<'a, Attachment> for PostMedia<'a> {
    const ENDPOINT: &'a str = "/api/v1/media";

    fn connection(&self) -> &Connection {
        self.conn
    }

    fn authorization(&self) -> Option<&str>{
        Some(self.conn.access_token())
    }
}

impl<'a> UploadInternal<'a, Attachment> for PostMedia<'a> {
    fn text_forms(&self) -> Vec<(&str, String)> {
        let mut forms: Vec<(&str, String)> = Vec::new();

        if let Some(description) = &self.description {
            forms.push(("description", description.into()));
        }

        if let Some(focus) = &self.focus {
            forms.push(("focus", focus.to_string()));
        }

        forms
    }

    fn file_form(&self) -> FileFormInternal<'_> {
        FileFormInternal {
            file_name: &self.filename,
            form_name: "file",
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize)]
struct Focus {
    x: f64,
    y: f64,
}

impl<'a> Focus {
    const MIN_FOCUS_VALUE: f64 = -1.0;
    const MAX_FOCUS_VALUE: f64 = 1.0;

    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    fn validate(&self) -> Result<()>{
        let range = Self::MIN_FOCUS_VALUE ..= Self::MAX_FOCUS_VALUE;

        if range.contains(&self.x) && range.contains(&self.y) {
            Ok(())
        } else {
            Err(
                Error::InvalidFocalPointError(self.x, self.y, Self::MIN_FOCUS_VALUE, Self::MAX_FOCUS_VALUE)
            )
        }
    }
}

use std::fmt;

impl fmt::Display for Focus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
