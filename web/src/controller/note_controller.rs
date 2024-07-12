use crate::controller::ApiResponse;
use crate::extractors::{
    authenticated_user::AuthenticatedUser, compare_api_version::CompareApiVersion,
};
use crate::{AppState, Error};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity_api::note as NoteApi;
use entity::notes::Model;
use service::config::ApiVersion;

use log::*;

/// POST create a new Note
#[utoipa::path(
    post,
    path = "/notes",
    request_body(content = entity_api::notes::Model, content_type = "application/json"),
    params(ApiVersion),
    responses(
        (status = 201, description = "Successfully Created a New Note", body = [entity::notes::Model]),
        (status= 422, description = "Unprocessable Entity"),
        (status = 401, description = "Unauthorized"),
        (status = 405, description = "Method not allowed")
    ),
    security(
        ("cookie_auth" = [])
    )
)]

pub async fn create(
    CompareApiVersion(_v): CompareApiVersion,
    AuthenticatedUser(_user): AuthenticatedUser,
    // TODO: create a new Extractor to authorize the user to access
    // the data requested
    State(app_state): State<AppState>,
    Json(note_model): Json<Model>,
) -> Result<impl IntoResponse, Error> {
    debug!("POST Create a New Note with from: {:?}", note_model);

    let note = NoteApi::create(app_state.db_conn_ref(), note_model).await?;

    debug!("New Note: {:?}", note);

    Ok(Json(ApiResponse::new(StatusCode::CREATED.into(), note)))
}
