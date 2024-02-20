/// This module provides features related to account used by moderators
use serde::Deserialize;
use crate::{
    DateTime,
    Utc,
};
use crate::entities::{
    Account as Acct,
    Application,
    Entity,
    Role,
    Admin_Ips,
};

/// Represents a user of Mastodon and their associated profile.
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Account {
    // Base attributes
    #[mastors(identifier)]
    id: String, // cast from an integer, but not guaranteed to be a number

    username: String,
    domain: Option<String>,
    created_at: DateTime<Utc>,
    email: String,
    ip: Option<String>,
    ips: Admin_Ips,
    locale: String,
    invite_request: Option<String>,
    role: Role,
    confirmed: bool,
    approved: bool,
    disabled: bool,
    silenced: bool,
    suspended: bool,
    account: Acct,
    created_by_application_id: Option<Application>,
    invited_by_account_id: Option<Acct>,
}

impl Account {
    /// Get an ID of this account.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the username of this account, not including domain.
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Get date time when this account was created.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn ip(&self) -> Option<&String> {
        self.ip.as_ref()
    }
    pub fn ips(&self) -> &Admin_Ips {
        &self.ips
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn invite_request(&self) -> Option<&String> {
        self.invite_request.as_ref()
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn approved(&self) -> bool {
        self.approved
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn silenced(&self) -> bool {
        self.silenced
    }

    pub fn suspended(&self) -> bool {
        self.suspended
    }

    pub fn account(&self) -> &Acct {
        &self.account
    }

    pub fn created_by_application_id(&self) -> Option<&Application> {
        self.created_by_application_id.as_ref()
    }

    pub fn invited_by_account_id(&self) -> Option<&Acct> {
        self.invited_by_account_id.as_ref()
    }
}

/// Represents an array of [`Account`](./struct.Account.html)s.
pub type Accounts = Vec<Account>;
impl Entity for Accounts {}
