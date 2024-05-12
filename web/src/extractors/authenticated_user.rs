use crate::extractors::RejectionType;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_login::AuthSession;
use entity::users;
use entity_api::user;

pub(crate) struct AuthenticatedUser(pub users::Model);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = RejectionType;

    // This extractor wraps the AuthSession extractor from axum_login. It extracts the user from the AuthSession and returns an AuthenticatedUser.
    // If the user is authenticated. If the user is not authenticated, it returns an Unauthorized error.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session: user::AuthSession = AuthSession::from_request_parts(parts, state)
            .await
            .map_err(|(status, msg)| (status, msg.to_string()))?;
        match session.user {
            Some(user) => Ok(AuthenticatedUser(user)),
            None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
        }
    }
}
