/// to use a case for i don't know what kind of json is returned from api.
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, mastors_derive::Entity)]
pub struct Dummy {}