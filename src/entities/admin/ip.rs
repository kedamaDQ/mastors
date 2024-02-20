use serde::Deserialize;
use crate::{
    DateTime,
    Utc,
};
use crate::entities::Entity;

/// Represents an ip address associated with a user
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Ip {
    #[mastors(identifier)]
    ip: String,

    used_at: DateTime<Utc>,
}

impl Ip {
    /// Get an ip
    pub fn ip(&self) -> &str {
        &self.ip
    }

    /// Get the timestamp of an ip last used
    pub fn used_at(&self) -> &DateTime<Utc> {
        &self.used_at
    }
}


pub type Ips = Vec<Ip>;
impl Entity for Ips {}