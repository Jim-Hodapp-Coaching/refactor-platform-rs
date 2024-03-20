use crate::AppState;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::HeaderValue, request::Parts, StatusCode},
};
use log::*;

pub static X_VERSION: &str = "x-version";

pub struct CheckApiVersion(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for CheckApiVersion
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    // A custom Extractor that extracts and checks that the API version number
    // provided in the "X-Version" header is equal to the API version specified
    // in AppState.
    // If this Extractor fails any Handler methods that use it will not be called
    // successfully.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        if let Some(version) = parts.headers.get(X_VERSION) {
            debug!("API version: {:?}", version);
            if state.config.api_version == Some(version.to_str().unwrap_or_default().into()) {
                debug!("Valid API version specified");
                Ok(CheckApiVersion(version.clone()))
            } else {
                error!(
                    "API version provided is not a valid API version: {:?}",
                    version
                );
                Err((
                    StatusCode::BAD_REQUEST,
                    "`X-Version` header is not a valid API version",
                ))
            }
        } else {
            error!("API version header not provided");
            Err((StatusCode::BAD_REQUEST, "`X-Version` header is missing"))
        }
    }
}
