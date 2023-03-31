use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::{Database, Sqlite};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

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
