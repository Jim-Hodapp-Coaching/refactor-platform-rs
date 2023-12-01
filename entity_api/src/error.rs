use std::error::Error as StdError;
use std::fmt;

use serde::Serialize;

use sea_orm::error::DbErr;

/// Errors while executing operations related to entities.
/// The intent is to categorize errors into two major types:
///  * Errors related to data. Ex DbError::RecordNotFound
///  * Errors related to interactions with the database itself. Ex DbError::Conn
#[derive(Debug)]
pub struct Error {
    // Underlying error emitted from seaORM internals
    pub inner: Option<DbErr>,
    // Enum representing which category of error
    pub error_code: EntityApiErrorCode,
}

#[derive(Debug, Serialize)]
pub enum EntityApiErrorCode {
    // Record not found
    RecordNotFound,
    // Record not updated
    RecordNotUpdated,
    // Errors related to interactions with the database itself. Ex DbError::Conn
    SystemError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity API Error: {:?}", self)
    }
}

impl StdError for Error {}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::RecordNotFound,
            },
            DbErr::RecordNotUpdated => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::RecordNotUpdated,
            },
            DbErr::ConnectionAcquire(_) => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::SystemError,
            },
            DbErr::Conn(_) => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::SystemError,
            },
            DbErr::Exec(_) => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::SystemError,
            },
            _ => Error {
                inner: Some(err),
                error_code: EntityApiErrorCode::SystemError,
            },
        }
    }
}
