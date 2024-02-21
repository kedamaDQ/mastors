//! This module provides features related to report problematic users to server moderators
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::{
        Report,
        ReportCategory,
    },
};

/// Create a POST request for /api/v1/reports
pub fn post(conn: &Connection, account_id: impl Into<String>) -> PostReports {
    PostReports {
        conn,
        account_id: account_id.into(),
        authorized: true,
        status_ids: None,
        comment: None,
        forward: false,
        category: ReportCategory::Other,
        rule_ids: None,
    }
}

/// POST request for /api/v1/reports
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Report, "/api/v1/reports")]
pub struct PostReports<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    authorized: bool,

    account_id: String,
    status_ids: Option<Vec<String>>,
    comment: Option<&'a str>,
    forward: bool,
    category: ReportCategory,
    rule_ids: Option<Vec<String>>,
}

impl<'a> Method<'a, Report> for PostReports<'a> {}

impl<'a> PostReports<'a> {
    /// Set status IDs if report includes some explicit statuses
    pub fn status_ids<T, U>(mut self, status_ids: T) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        self.status_ids = StringIds::new(status_ids).string_ids();
        self
    }

    /// Set report comment if any
    pub fn comment(mut self, comment: &'a str) -> Self {
        self.comment = if comment.is_empty() {
            None
        } else {
            Some(comment)
        };
        self
    }

    /// Set to transfer report to origin server (don't transfer by default)
    pub fn with_forward(mut self) -> Self {
        self.forward = true;
        self
    }

    /// Set to don't transfer report to origin server (don't transfer by default)
    pub fn without_forward(mut self) -> Self {
        self.forward = false;
        self
    }

    /// Set report category (Other by default)
    pub fn category(mut self, category: ReportCategory) -> Self {
        self.category = category;
        self
    }

    /// Set rule IDs if category is Violation
    pub fn rule_ids<T, U>(mut self, rules_ids: T) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        self.rule_ids = StringIds::new(rules_ids).string_ids();
        self
    }
}

use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct StringIds {
    string_ids: Vec<String>,
}

impl StringIds {
    fn new<T, U>(string_ids: T) -> Self
    where
        T: AsRef<[U]>,
        U: AsRef<str>,
    {
        let string_ids = string_ids.as_ref()
            .iter()
            .map(|u| u.as_ref().trim())
            .filter(|u| !u.is_empty())
            .map(|u| u.to_owned())
            .collect::<HashSet<String>>()
            .iter()
            .map(|u| u.to_owned())
            .collect::<Vec<String>>();

        StringIds {
            string_ids,
        }
    }

    fn string_ids(&self) -> Option<Vec<String>> {
        if self.string_ids.is_empty() {
            None
        } else {
            Some(self.string_ids.clone())
        }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.string_ids.len()
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.string_ids.is_empty()
    }
}
