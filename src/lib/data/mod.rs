use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, From, Display, Serialize, Deserialize)]
pub struct Dbid(Uuid);
impl Dbid {
    pub fn new() -> Dbid {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Dbid {
        Self(Uuid::nil())
    }
}

impl Default for Dbid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Dbid {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dbid(Uuid::parse_str(s)?))
    }
}
