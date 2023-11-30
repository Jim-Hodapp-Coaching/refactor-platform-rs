use std::error::Error as StdError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use entity_api::error::EntityApiErrorType;
use entity_api::error::Error as EntityApiError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]

pub enum Error {
    DatabaseConnectionLost,
    InternalServer,
    EntityNotFound,
    UnprocessableEntity,
}

impl StdError for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DatabaseConnectionLost => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DB CONNECTION LOST - INTERNAL SERVER ERROR",
            )
                .into_response(),
            Error::InternalServer => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR").into_response()
            }
            Error::EntityNotFound => (StatusCode::NOT_FOUND, "ENTITY NOT FOUND").into_response(),
            Error::UnprocessableEntity => {
                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE ENTITY").into_response()
            }
        }
    }
}

impl From<EntityApiError> for Error {
    fn from(err: EntityApiError) -> Self {
        match err.error_type {
            EntityApiErrorType::DatabaseConnectionLost => Error::DatabaseConnectionLost,
            EntityApiErrorType::RecordNotFound => Error::EntityNotFound,
            EntityApiErrorType::RecordNotUpdated => Error::UnprocessableEntity,
            EntityApiErrorType::SystemError => Error::InternalServer,
        }
    }
}
