/// this module represents a custom user role
use serde::Deserialize;

/// Represents a custom user role
#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Role {
    #[mastors(identifier)]
    id: String,

    name: String,
    color: String,
    permissions: String,
    highlighted: bool,
}

impl Role {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn permissions(&self) -> &str {
        &self.permissions
    }

    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }
}

#[allow(unused)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(u32)]
pub enum Permission {
    Administrator       = 0x00000001,
    Devops              = 0x00000002,
    ViewAuditLog        = 0x00000004,
    ViewDashboard       = 0x00000008,
    ManageReports       = 0x00000010,
    ManageFederation    = 0x00000020,
    ManageSettings      = 0x00000040,
    ManageBlocks        = 0x00000080,
    ManageTaxonomies    = 0x00000100,
    ManageAppeals       = 0x00000200,
    ManageUsers         = 0x00000400,
    ManageInvites       = 0x00000800,
    ManageRules         = 0x00001000,
    ManageAnnouncements = 0x00002000,
    ManageCustomEmojis  = 0x00004000,
    ManageWebhooks      = 0x00008000,
    InviteUsers         = 0x00010000,
    ManageRoles         = 0x00020000,
    ManageUserAccess    = 0x00040000,
    DeleteUserData      = 0x00080000,
}

/*
use std::fmt;

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Permission::Administrator       => write!(f, 0x00000001),
            Permission::Devops              => write!(f, 0x00000002),
            Permission::ViewAuditLog        => write!(f, 0x00000004),
            Permission::ViewDashboard       => write!(f, 0x00000008),
            Permission::ManageReports       => write!(f, 0x00000010),
            Permission::ManageFederation    => write!(f, 0x00000020),
            Permission::ManageSettings      => write!(f, 0x00000040),
            Permission::ManageBlocks        => write!(f, 0x00000080),
            Permission::ManageTaxonomies    => write!(f, 0x00000100),
            Permission::ManageAppeals       => write!(f, 0x00000200),
            Permission::ManageUsers         => write!(f, 0x00000400),
            Permission::ManageInvites       => write!(f, 0x00000800),
            Permission::ManageRules         => write!(f, 0x00001000),
            Permission::ManageAnnouncements => write!(f, 0x00002000),
            Permission::ManageCustomEmojis  => write!(f, 0x00004000),
            Permission::ManageWebhooks      => write!(f, 0x00008000),
            Permission::InviteUsers         => write!(f, 0x00010000),
            Permission::ManageRoles         => write!(f, 0x00020000),
            Permission::ManageUserAccess    => write!(f, 0x00040000),
            Permission::DeleteUserData      => write!(f, 0x00080000),
        }
    }
}

use serde::{ de };

impl<'de> de::Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> std::result<Self, D::Error>
    where 
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Permission
    }

}
#[cfg(test)]
mod test {
    use serde_json::*;
    use super::*;

    #[test]
    fn test_from_numeric_literal() {
        let role: Role = serde_json::from_str(data).unwrap();
    }

    const data:str = r#"
        {
            "id": "1",
            "name": "name",
            "color": "ffffff"
            "permissions": 2,
            "highlighted: true
        }
    "#;
}
*/