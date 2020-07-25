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
