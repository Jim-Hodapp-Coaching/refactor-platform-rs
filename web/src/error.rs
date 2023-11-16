use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalServer,
    EntityNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InternalServer => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR").into_response()
            }
            Error::EntityNotFound => (StatusCode::NOT_FOUND, "ENTITY NOT FOUND").into_response(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
