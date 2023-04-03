use crate::domain::clip::field;
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        Self {
            shortcode: value,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self::from_raw(value)
    }
}
