pub mod field;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("hints parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub clip_id: field::ClipId,      // The id of the clip
    pub shortcode: field::ShortCode, // The url-code to access a clip
    pub content: field::Content,     // The content of a clip
    pub title: field::Title,         // The title of a clip
    pub posted: field::Posted,       // The date that a clip is posted
    pub expires: field::Expires,     // The date that a clip expires
    pub password: field::Password,   // The password to access a clip
    pub hits: field::Hits,           // Number of times the clip has been seen
}
