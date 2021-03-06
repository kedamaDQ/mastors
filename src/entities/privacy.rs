/// Represents a privacy setting of the status.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

use std::fmt;
use std::str::FromStr;

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

impl FromStr for Privacy {
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

use serde::{ ser, de };

impl ser::Serialize for Privacy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for Privacy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
		let s = String::deserialize(deserializer)?;
		match Privacy::from_str(s.as_str()) {
			Ok(r) => Ok(r),
			Err(e) => Err(de::Error::custom(e)),
		}
    }

}
/// Represents a visibility of the status.
/// 
/// This enum is an alias of `Privacy`
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
        assert_eq!("'privatte' is not a valid Privacy string", result);
    }

}
