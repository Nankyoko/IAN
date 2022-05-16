use std::fmt::Display;

use actix_web::{error::BlockingError, ResponseError};

#[derive(Debug)]
pub struct DbError {
    inner_err: anyhow::Error
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_err.fmt(f)
    }
}

impl From<anyhow::Error> for DbError {
    fn from(err: anyhow::Error) -> Self {
        DbError { inner_err: err }
    }
}

impl From<BlockingError> for DbError {
    fn from(err: BlockingError) -> Self {
        DbError {
            inner_err: anyhow::anyhow!("{}", err.to_string())
        }
    }
}

impl ResponseError for DbError{}