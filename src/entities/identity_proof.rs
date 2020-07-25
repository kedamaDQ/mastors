use serde::Deserialize;
use crate::{
    DateTime,
    Url,
    Utc,
};
use super::Entity;

/// Represents a proof from an external identity provider.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, mastors_derive::Entity)]
pub struct IdentityProof {
    provider: String,
    provider_username: String,
    profile_url: Url,
    proof_url: Url,
    updated_at: DateTime<Utc>,
}

impl IdentityProof {
    /// Get the name of the identity provider.
    pub fn provider(&self) -> &str {
        self.provider.as_str()
    }

    /// Get the account owner's name on the identity provider.
    pub fn provider_username(&self) -> &str {
        self.provider_username.as_str()
    }

    /// Get the account owner's profile url on the identity provider.
    pub fn profile_url(&self) -> &Url {
        &self.profile_url
    }

    /// Get a link to a statement of identity proof, hosted by the identity provider.
    pub fn proof_url(&self) -> &Url {
        &self.proof_url
    }

    /// Get updated date and time of the account owner on the identity provider.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// Represents an array of [`IdentityProof`](./struct.IdentityProof.html)s.
pub type IdentityProofs = Vec<IdentityProof>;
impl Entity for IdentityProofs {}
