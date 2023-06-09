use crate::data::Dbid;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct ClipId(Dbid);
impl ClipId {
    pub fn into_inner(self) -> Dbid {
        self.0
    }
}

impl From<Dbid> for ClipId {
    fn from(value: Dbid) -> Self {
        Self(value)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(Dbid::nil())
    }
}
