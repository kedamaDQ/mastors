//! This module provides features related to uploading media attached to status.
use serde::Serialize;
use crate::{
    Connection,
    Error,
    Method,
    Result,
    entities::Attachment,
    private::{
        UploadInternal,
        FileFormInternal,
    }
};

/// Get a request to upload the media that will attach to status.
pub fn post(conn: &Connection, filename: impl Into<String>) -> PostMedia {
    PostMedia {
        conn,
        auth: true,
        filename: filename.into(),
        description: None,
        focus: None,
        focus_str: None,
    }
}

/// POST request for `/api/v1/media`.
#[derive(Debug, Serialize, mastors_derive::Method)]
#[method_params(POST, Attachment, "/api/v1/media")]
pub struct PostMedia<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    filename: String,
    description: Option<String>,
    focus: Option<Focus>,
    focus_str: Option<String>,
}

impl<'a> PostMedia<'a> {
    /// Set a description text of this media attachment.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the focal point of this media attachment when viewed in a cropped thumbnail view.
    pub fn focus(mut self, x: f64, y: f64) -> Self {
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
        self.post_with_media()
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

impl Focus {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_image() {
        let conn = Connection::new().unwrap();
        let posted = post(&conn, "./test-resources/test1.png")
            .description("bar board")
            .focus(0f64, 1.0f64)
            .send()
            .unwrap();
        
        println!("{:#?}", posted);

        assert_eq!(posted.description().unwrap(), "bar board");
        /* Floating point calculations are usually imprecise

        assert_eq!(posted.meta().unwrap().focus().unwrap().x(), 0f64);
        assert_eq!(posted.meta().unwrap().focus().unwrap().y(), 1.0f64);
        */
    }

    #[test]
    fn test_focus_to_fail_to_validation() {
        assert!(Focus::new(-1.0, 1.0).validate().is_ok());
        assert!(Focus::new(-1.0001, 0.0).validate().is_err());
        assert!(Focus::new(1.0001, 0.0).validate().is_err());
        assert!(Focus::new(0.0, -1.0001).validate().is_err());
        assert!(Focus::new(0.0, 1.0001).validate().is_err());
    }
}
