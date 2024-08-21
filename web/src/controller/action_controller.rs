use crate::controller::ApiResponse;
use crate::extractors::{
    authenticated_user::AuthenticatedUser, compare_api_version::CompareApiVersion,
};
use crate::{AppState, Error};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity::{actions::Model, Id};
use entity_api::action as ActionApi;
use service::config::ApiVersion;
use std::collections::HashMap;

use log::*;

/// POST create a new Action
#[utoipa::path(
    post,
    path = "/actions",
    params(ApiVersion),
    request_body = entity::actions::Model,
    responses(
        (status = 201, description = "Successfully Created a New Action", body = [entity::actions::Model]),
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
    AuthenticatedUser(user): AuthenticatedUser,
    // TODO: create a new Extractor to authorize the user to access
    // the data requested
    State(app_state): State<AppState>,
    Json(action_model): Json<Model>,
) -> Result<impl IntoResponse, Error> {
    debug!("POST Create a New Action from: {:?}", action_model);

    let action = ActionApi::create(app_state.db_conn_ref(), action_model, user.id).await?;

    debug!("New Action: {:?}", action);

    Ok(Json(ApiResponse::new(StatusCode::CREATED.into(), action)))
}

/// GET a particular Action specified by its id.
#[utoipa::path(
    get,
    path = "/actions/{id}",
    params(
        ApiVersion,
        ("id" = String, Path, description = "Action id to retrieve")
    ),
    responses(
        (status = 200, description = "Successfully retrieved a specific Action by its id", body = [entity::notes::Model]),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Note not found"),
        (status = 405, description = "Method not allowed")
    ),
    security(
        ("cookie_auth" = [])
    )
)]
pub async fn read(
    CompareApiVersion(_v): CompareApiVersion,
    State(app_state): State<AppState>,
    Path(id): Path<Id>,
) -> Result<impl IntoResponse, Error> {
    debug!("GET Action by id: {}", id);

    let note: Option<Model> = ActionApi::find_by_id(app_state.db_conn_ref(), id).await?;

    Ok(Json(ApiResponse::new(StatusCode::OK.into(), note)))
}

#[utoipa::path(
    put,
    path = "/actions/{id}",
    params(
        ApiVersion,
        ("id" = Id, Path, description = "Id of action to update"),
    ),
    request_body = entity::actions::Model,
    responses(
        (status = 200, description = "Successfully Updated Action", body = [entity::actions::Model]),
        (status = 401, description = "Unauthorized"),
        (status = 405, description = "Method not allowed")
    ),
    security(
        ("cookie_auth" = [])
    )
)]
pub async fn update(
    CompareApiVersion(_v): CompareApiVersion,
    AuthenticatedUser(_user): AuthenticatedUser,
    // TODO: create a new Extractor to authorize the user to access
    // the data requested
    State(app_state): State<AppState>,
    Path(id): Path<Id>,
    Json(action_model): Json<Model>,
) -> Result<impl IntoResponse, Error> {
    debug!("PUT Update Action with id: {}", id);

    let action = ActionApi::update(app_state.db_conn_ref(), id, action_model).await?;

    debug!("Updated Action: {:?}", action);

    Ok(Json(ApiResponse::new(StatusCode::OK.into(), action)))
}

#[utoipa::path(
    get,
    path = "/actions",
    params(
        ApiVersion,
        ("coaching_session_id" = Option<Id>, Query, description = "Filter by coaching_session_id")
    ),
    responses(
        (status = 200, description = "Successfully retrieved all Actions", body = [entity::actions::Model]),
        (status = 401, description = "Unauthorized"),
        (status = 405, description = "Method not allowed")
    ),
    security(
        ("cookie_auth" = [])
    )
)]
pub async fn index(
    CompareApiVersion(_v): CompareApiVersion,
    AuthenticatedUser(_user): AuthenticatedUser,
    // TODO: create a new Extractor to authorize the user to access
    // the data requested
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    debug!("GET all Actions");
    debug!("Filter Params: {:?}", params);

    let actions = ActionApi::find_by(app_state.db_conn_ref(), params).await?;

    debug!("Found Actions: {:?}", actions);

    Ok(Json(ApiResponse::new(StatusCode::OK.into(), actions)))
}
