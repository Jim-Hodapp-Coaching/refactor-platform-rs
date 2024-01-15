use crate::Error;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use entity_api::user as UserApi;
use log::*;
use serde::Deserialize;
use serde_json::json;

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub struct UserSessionController {}

impl UserSessionController {
    pub async fn protected(auth_session: UserApi::AuthSession) -> impl IntoResponse {
        debug!("UserSessionController::protected()");

        match auth_session.user {
            Some(user) => json!({
                "email": &user.email,
            })
            .to_string()
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    /// Create a new user session
    /// Test this with curl: curl \
    /// --request GET \
    /// http://localhost:4000/login
    pub async fn get_login(
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> Result<impl IntoResponse, Error> {
        debug!(
            "UserSessionController::get_login(), next: {:?}",
            next.unwrap_or_default()
        );

        // TODO: try and respond with an HTML template like the example until we understand
        // how the code works inside and out.

        Ok(())
    }

    /// curl -v --header "Content-Type: application/x-www-form-urlencoded" \
    /// --data "username=james.hodapp@gmail.com&password=password1&next=organizations" \
    /// http://localhost:4000/login
    ///
    /// Successful login will return a session cookie with id, e.g.:
    /// set-cookie: id=07bbbe54-bd35-425f-8e63-618a8d8612df; HttpOnly; SameSite=Strict; Path=/; Max-Age=86399
    ///
    /// After logging in successfully, you must pass the session id back to the server for
    /// every API call, e.g.:
    /// curl -v --header "Cookie: id=07bbbe54-bd35-425f-8e63-618a8d8612df" --request GET http://localhost:4000/organizations
    pub async fn login(
        mut auth_session: UserApi::AuthSession,
        Form(creds): Form<UserApi::Credentials>,
    ) -> impl IntoResponse {
        debug!("UserSessionController::login()");

        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(StatusCode::UNAUTHORIZED.into_response()),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };

        if auth_session.login(&user).await.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }

        if let Some(ref next) = creds.next {
            debug!("Redirecting to next: {next}");
            Ok(Redirect::to(next).into_response())
        } else {
            debug!("Redirecting to root");
            Ok(Redirect::to("/").into_response())
        }
    }
}
