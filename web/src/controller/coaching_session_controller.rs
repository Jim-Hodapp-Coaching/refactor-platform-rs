use crate::controller::ApiResponse;
use crate::extractors::{
    authenticated_user::AuthenticatedUser, compare_api_version::CompareApiVersion,
};
use crate::{AppState, Error};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity_api::coaching_session as CoachingSessionApi;
use service::config::ApiVersion;
use std::collections::HashMap;

use log::*;

#[utoipa::path(
    get,
    path = "/coaching_sessions",
    params(
        ApiVersion,
        ("coaching_relationship_id" = Option<Id>, Query, description = "Filter by coaching_relationship_id"),
        ("from_date" = Option<NaiveDate>, Query, description = "Filter by from_date"),
        ("to_date" = Option<NaiveDate>, Query, description = "Filter by to_date")
    ),
    responses(
        (status = 200, description = "Successfully retrieved all Coaching Sessions", body = [entity::coaching_sessions::Model]),
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
    debug!("GET all Coaching Sessions");
    debug!("Filter Params: {:?}", params);

    let coaching_sessions = CoachingSessionApi::find_by(app_state.db_conn_ref(), params).await?;

    debug!("Found Coaching Sessions: {:?}", coaching_sessions);

    Ok(Json(ApiResponse::new(
        StatusCode::OK.into(),
        coaching_sessions,
    )))
}
