use std::error::Error as StdError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use entity_api::error::EntityApiErrorCode;
use entity_api::error::Error as EntityApiError;

extern crate log;
use log::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(EntityApiError);

impl StdError for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// List of possible StatusCode variants https://docs.rs/http/latest/http/status/struct.StatusCode.html#associatedconstant.UNPROCESSABLE_ENTITY
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self.0.error_code {
            EntityApiErrorCode::SystemError => {
                debug!("Error: {:#?}, mapping to INTERNAL_SERVER_ERROR", self);

                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR").into_response()
            }
            EntityApiErrorCode::RecordNotFound => {
                debug!("Error: {:#?}, mapping to NO_CONTENT", self);

                (StatusCode::NO_CONTENT, "NO CONTENT").into_response()
            }
            EntityApiErrorCode::RecordNotUpdated => {
                debug!("Error: {:#?}, mapping to UNPROCESSABLE_ENTITY", self);

                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE ENTITY").into_response()
            }
            EntityApiErrorCode::RecordUnauthenticated => {
                debug!("Error: {:#?}, mapping to UNAUTHORIZED", self);

                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response()
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
