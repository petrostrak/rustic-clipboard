use crate::data::Dbid;
use crate::{ClipError, ShortCode, Time};
use chrono::NaiveDateTime;
use std::convert::TryFrom;
use std::str::FromStr;

// model::Clip encapsulates DB specific information
// and is used only when interacting with the DB
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

// Converting model::Clip into domain::Clip
impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok(Self {
            clip_id: field::ClipId::new(Dbid::from_str(value.clip_id.as_str())?),
            shortcode: field::ShortCode::from(value.shortcode),
            content: field::Content::new(value.content.as_str())?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_naive_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_naive_utc)),
            password: field::Password::new(value.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}

// GetClip retrieves a clip from DB
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(value: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        GetClip {
            shortcode: value.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(value: String) -> Self {
        GetClip { shortcode: value }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i16,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
}

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}
