use crate::{AppState, Error};
use axum::extract::State;
use axum::{http::StatusCode, response::{IntoResponse, Redirect}};
use axum::{Form, Json};
use entity::user;
use entity_api::user as UserApi;
use log::*;
use serde_json::json;

pub struct UserSessionController {}

impl UserSessionController {
    pub async fn protected(auth_session: UserApi::AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => json!({
                "email": &user.email,
            }).to_string().into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    /// Create a new user session
    /// Test this with curl: curl \
    /// --request GET \
    /// http://localhost:4000/
    pub async fn index() -> Result<impl IntoResponse, Error> {
        debug!("UserSessionController::index()");

        Ok(())
    }

    /// Create a new user session
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"email":"blah@test.com"}' \
    /// http://localhost:4000/login/password
    /// 
    /// curl --location 'localhost:4000/login/password' \
    /// --form 'username="username"' \
    /// --form 'password="password"'
    pub async fn login(
        mut auth_session: UserApi::AuthSession,
        Form(creds): Form<UserApi::Credentials>,
    ) -> impl IntoResponse {
        debug!("user_session_controller::login()");

        let user = match auth_session
                .authenticate(creds.clone())
                .await
        {
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