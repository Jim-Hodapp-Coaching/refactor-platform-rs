use std::error::Error as StdError;
use std::fmt;

use sea_orm::error::DbErr;

/// Errors while executing operations related to entities.
/// The intent is to categorize errors into two major types:
///  * Errors related to data. Ex DbError::RecordNotFound
///  * Errors related to interactions with the database itself. Ex DbError::Conn
#[derive(Debug)]
pub struct Error {
    // Underlying error emitted from seaORM internals
    pub inner: DbErr,
    // Enum representing which category of error
    pub error_type: EntityApiError,
}

#[derive(Debug)]
pub enum EntityApiError {
    DatabaseConnectionLost,
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
                inner: err,
                error_type: EntityApiError::RecordNotFound,
            },
            DbErr::RecordNotUpdated => Error {
                inner: err,
                error_type: EntityApiError::RecordNotUpdated,
            },
            DbErr::ConnectionAcquire(_) => Error {
                inner: err,
                error_type: EntityApiError::SystemError,
            },
            DbErr::Conn(_) => Error {
                inner: err,
                error_type: EntityApiError::DatabaseConnectionLost,
            },
            DbErr::Exec(_) => Error {
                inner: err,
                error_type: EntityApiError::SystemError,
            },
            _ => Error {
                inner: err,
                error_type: EntityApiError::SystemError,
            },
        }
    }
}