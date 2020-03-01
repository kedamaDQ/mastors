use serde::Deserialize;
use super::{
  Status,
  Entity,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct Context {
  // Required attributes
  ancestors: Vec<Status>,
  descendants: Vec<Status>,
}

impl Entity for Context {}
