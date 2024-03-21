use crate::AppState;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::HeaderValue, request::Parts, StatusCode},
};
use log::*;

type RejectionType = (StatusCode, &'static str);

pub static X_VERSION: &str = "x-version";

pub struct CheckApiVersion(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for CheckApiVersion
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = RejectionType;

    // A custom Extractor that extracts and checks that the API version number
    // provided in the "X-Version" header is equal to the API version specified
    // in AppState.
    // If this Extractor fails any Handler methods that use it will not be called
    // successfully.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let version = get_x_version(parts)?;
        let api_version = HeaderValue::from_str(&state.config.api_version.unwrap_or_default()).ok().unwrap();

        trace!("API version provided by client: {:?}", version);
        trace!("API version set in AppState.config.api_version: {:?}", version);

        Ok(is_current_api_version(version, api_version)?)
    }
}

fn get_x_version(parts: &mut Parts) -> Result<HeaderValue, RejectionType> {
    if let Some(version) = parts.headers.get(X_VERSION) {
        Ok(version.clone())
    } else {
        Err((StatusCode::BAD_REQUEST, "`X-Version` header is missing"))
    }
}

fn is_current_api_version(version: HeaderValue, api_version: HeaderValue) -> Result<CheckApiVersion, RejectionType> {
    if version == api_version {
        Ok(CheckApiVersion(version))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            "`X-Version` header is not a valid API version",
        ))
    }
}
