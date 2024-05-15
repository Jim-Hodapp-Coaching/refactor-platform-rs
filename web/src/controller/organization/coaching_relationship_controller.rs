use crate::controller::ApiResponse;
use crate::extractors::{
    authenticated_user::AuthenticatedUser, compare_api_version::CompareApiVersion,
};
use crate::{AppState, Error};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity_api::coaching_relationship as CoachingRelationshipApi;

use log::*;

/// GET all CoachingRelationships by organization_id
pub async fn index(
    CompareApiVersion(_v): CompareApiVersion,
    AuthenticatedUser(_user): AuthenticatedUser,
    // TODO: create a new Extractor to authorize the user to access
    // the data requested
    State(app_state): State<AppState>,
    Path(organization_id): Path<String>,
) -> Result<impl IntoResponse, Error> {
    debug!("GET all CoachingRelationships");
    let coaching_relationships =
        CoachingRelationshipApi::find_by_organization(app_state.db_conn_ref(), organization_id)
            .await?;

    debug!("Found CoachingRelationships: {:?}", coaching_relationships);

    Ok(Json(ApiResponse::new(
        StatusCode::OK.as_u16(),
        coaching_relationships,
    )))
}
