use serde::Deserialize;
use crate::{
    DateTime,
    Error,
    Utc,
};
use crate::entities::{
    Entity,
    Rules,
    Statuses,
    Admin_Account as AdminAccount,
};

/// Represents a report about problematic users
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Report {
    #[mastors(identifier)]
    id: String,

    action_taken: bool,
    action_taken_at: Option<DateTime<Utc>>,
    category: String,
    comment: String,
    forwarded: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    account: AdminAccount,
    target_account: AdminAccount,
    assigned_account: Option<AdminAccount>,
    action_taken_by_account: Option<AdminAccount>,
    statuses: Statuses,
    rules: Rules,
}

impl Report {
    /// Get an ID of the report
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get whether any action was taken fot the report
    pub fn action_taken(&self) -> bool {
        self.action_taken
    }

    /// Get timestamp when any action was taken for the report
    pub fn action_taken_at(&self) -> Option<&DateTime<Utc>> {
        self.action_taken_at.as_ref()
    }

    /// Get report category e.g. spam, violation, other
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Get comment for the report
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Get whether the report was transfered to origin server
    pub fn forwarded(&self) -> bool {
        self.forwarded
    }

    /// Get thimestamp when create the report
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Get timestamp when update the report
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Get an account which filed the report
    pub fn account(&self) -> &AdminAccount {
        &self.account
    }

    /// Get the account being reported
    pub fn target_account(&self) -> &AdminAccount {
        &self.target_account
    }

    /// Get the moderator account assigned to the report
    pub fn assigned_account(&self) -> Option<&AdminAccount> {
        self.assigned_account.as_ref()
    }

    /// Get the moderator account who handled the report
    pub fn action_taken_by_account(&self) -> Option<&AdminAccount> {
        self.action_taken_by_account.as_ref()
    }

    /// Get reported status IDs if any
    pub fn statuses(&self) -> &Statuses {
        self.statuses.as_ref()
    }

    /// Get server rule IDs in case of category is violation
    pub fn rules(&self) -> &Rules {
        self.rules.as_ref()
    }
}

/// Represents an array of [`Report`](./struct.Report.html)s
pub type Reports = Vec<Report>;
impl Entity for Reports {}

/// Represents a kind of report categoris
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ReportCategory {
    /// this is spam
    Spam,

    /// rule vioalation
    Violation,

    /// other
    Other,
}

use std::fmt;

impl fmt::Display for ReportCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportCategory::Spam => write!(f, "spam"),
            ReportCategory::Violation => write!(f, "violation"),
            ReportCategory::Other => write!(f, "other"),
        }
    }
}

use std::str::FromStr;

impl FromStr for ReportCategory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spam" => Ok(ReportCategory::Spam),
            "violation" => Ok(ReportCategory::Violation),
            "other" => Ok(ReportCategory::Other),
            _ => Err(Error::ParseReportCategoryError(s.to_owned())),
        }
    }
}

use serde::{ ser, de };

impl ser::Serialize for ReportCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for ReportCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match ReportCategory::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}