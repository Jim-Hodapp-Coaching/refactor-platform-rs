use std::error::Error as StdError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use entity_api::error::EntityApiErrorType;
use entity_api::error::Error as EntityApiError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(EntityApiError);

impl StdError for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self.0.error_type {
            EntityApiErrorType::SystemError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR").into_response()
            }
            EntityApiErrorType::RecordNotFound => {
                (StatusCode::NO_CONTENT, "NO CONTENT").into_response()
            }
            EntityApiErrorType::RecordNotUpdated => {
                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE ENTITY").into_response()
            }
        }
    }
}

impl<E> From<E> for Error
where
    E: Into<EntityApiError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
