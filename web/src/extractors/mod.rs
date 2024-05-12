pub(crate) mod authenticated_user;
pub(crate) mod compare_api_version;

use axum::http::StatusCode;

type RejectionType = (StatusCode, String);
