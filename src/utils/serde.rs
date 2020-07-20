use std::str::FromStr;
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

pub(crate) fn transform_str_to_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error> 
where
    D: Deserializer<'de>,
    T: FromStr<Err = crate::Error>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    T::from_str(s).map_err(D::Error::custom)
}

pub(crate) fn transform_option_str_to_enum<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr<Err = crate::Error>,
{
    let o: Option<&str> = Deserialize::deserialize(deserializer)?;
    match o {
        Some(s) => {
            match T::from_str(s).map_err(D::Error::custom) {
                Ok(en) => Ok(Some(en)),
                Err(e) => Err(e),
            }
        },
        None => Ok(None)
    }
}
