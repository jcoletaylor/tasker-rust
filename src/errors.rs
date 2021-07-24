use redis::RedisError;
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryError {
    pub error_string: String,
}

impl RepositoryError {
    pub fn new(error_string: String) -> Self {
        Self { error_string }
    }
}

impl From<SqlxError> for RepositoryError {
    fn from(error: SqlxError) -> Self {
        Self {
            error_string: error.to_string(),
        }
    }
}

impl From<RedisError> for RepositoryError {
    fn from(error: RedisError) -> Self {
        Self {
            error_string: error.to_string(),
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_string)
    }
}

impl Error for RepositoryError {
    fn description(&self) -> &str {
        &self.error_string
    }
}
