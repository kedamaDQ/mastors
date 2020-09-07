use serde::{
    Deserialize,
    Deserializer,
    de::Error,
};

pub(crate) fn transform_string_to_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>
{

    let s: &str = Deserialize::deserialize(deserializer)?;
    usize::from_str_radix(s, 10).map_err(D::Error::custom)
}

pub(crate) fn transform_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>
{

    let s: &str = Deserialize::deserialize(deserializer)?;
    u64::from_str_radix(s, 10).map_err(D::Error::custom)
}

pub(crate) fn transform_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>
{

    let s: &str = Deserialize::deserialize(deserializer)?;
    i64::from_str_radix(s, 10).map_err(D::Error::custom)
}

use crate::Url;

pub(crate) fn transform_string_to_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D:Deserializer<'de>
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.to_string()))
    }
}
pub(crate) fn transform_string_to_option_url<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: Deserializer<'de>,
{

    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Url::parse(s).map_err(D::Error::custom)?))
    }
}
