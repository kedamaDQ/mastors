use std::fmt;
use serde::{
    Serialize,
    Deserialize,
};

/// Represents a visibility of the status.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Privacy {
    /// Visible to everyone, shown in public timelines.
    Public,

    /// Visible to public but not included in public timelines.
    Unlisted,

    /// Visible to followers only, and to any mentioned users.
    Private,

    /// Visible only to mentioned users.
    Direct,
}

impl fmt::Display for Privacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Privacy::Public => write!(f, "public"),
            Privacy::Unlisted => write!(f, "unlisted"),
            Privacy::Private => write!(f, "private"),
            Privacy::Direct => write!(f, "direct"),
        }
    }
}

impl std::str::FromStr for Privacy {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Privacy::Public),
            "unlisted" => Ok(Privacy::Unlisted),
            "private" => Ok(Privacy::Private),
            "direct" => Ok(Privacy::Direct),
            _ => Err(crate::Error::ParsePrivacyError(s.to_string())),
        }
    }
}

/// Represents a visibility of the status.
pub type Visibility = Privacy;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!("public", Privacy::Public.to_string());
    }

    #[test]
    fn from_valid_string() {
        assert_eq!(Privacy::Unlisted, Privacy::from_str("unlisted").unwrap());
    }

    #[test]
    fn from_invalid_string() {
        let privacy = "privatte";
        let result = match Privacy::from_str(privacy) {
            Ok(_) => String::new(),
            Err(e) => e.to_string(),
        };
        assert_eq!("'privatte' is not a valid privacy.", result);
    }

}
