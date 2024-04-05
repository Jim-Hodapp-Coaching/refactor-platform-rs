use crate::controller::ApiResponse;
use crate::extractors::{
    authenticated_user::AuthenticatedUser, compare_api_version::CompareApiVersion,
};
use crate::{AppState, Error};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use entity_api::coaching_relationship as CoachingRelationshipApi;
use std::collections::HashMap;

use log::*;

pub struct CoachingRelationshipsController {}

impl CoachingRelationshipsController {
    /// GET all CoachingRelationships
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request GET \
    /// http://localhost:4000/coaching_relationships
    pub async fn index(
        CompareApiVersion(_v): CompareApiVersion,
        AuthenticatedUser(_user): AuthenticatedUser,
        // TODO: create a new Extractor to authorize the user to access
        // the data requested
        State(app_state): State<AppState>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET all CoachingRelationships");
        let coaching_relationships =
            CoachingRelationshipApi::find_by(app_state.db_conn_ref(), params).await?;

        debug!("Found CoachingRelationships: {:?}", coaching_relationships);

        Ok(Json(ApiResponse::new(
            StatusCode::OK.as_u16(),
            coaching_relationships,
        )))
    }
}
