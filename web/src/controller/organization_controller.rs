use crate::extractors::{
    compare_api_version::CompareApiVersion,
    authenticated_user::AuthenticatedUser,
};
use crate::{AppState, Error};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::{organizations, Id};
use entity_api::organization as OrganizationApi;
use serde_json::json;
use std::collections::HashMap;

use log::*;

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations
    pub async fn index(
        CompareApiVersion(_v): CompareApiVersion,
        AuthenticatedUser(_user): AuthenticatedUser,
        // TODO: create a new Extractor to authorize the user to access
        // the data requested
        State(app_state): State<AppState>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET all Organizations");
        let organizations = OrganizationApi::find_by(app_state.db_conn_ref(), params).await?;

        debug!("Found Organizations: {:?}", organizations);

        Ok(Json(organizations))
    }

    /// GET a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations/<id>
    pub async fn read(
        CompareApiVersion(_v): CompareApiVersion,
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET Organization by id: {}", id);

        let organization: Option<organizations::Model> =
            OrganizationApi::find_by_id(app_state.db_conn_ref(), id).await?;

        Ok(Json(organization))
    }

    /// CREATE a new Organization entity
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"name":"My New Organization"}' \
    /// http://localhost:4000/organizations
    pub async fn create(
        CompareApiVersion(_v): CompareApiVersion,
        State(app_state): State<AppState>,
        Json(organization_model): Json<organizations::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("CREATE new Organization: {:?}", organization_model.name);

        let organization: organizations::Model =
            OrganizationApi::create(app_state.db_conn_ref(), organization_model).await?;

        debug!("Newly Created Organization: {:?}", &organization);

        Ok(Json(organization))
    }

    /// UPDATE a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request PUT  http://localhost:4000/organizations/<id> \
    /// --data '{"name":"My Updated Organization"}'
    pub async fn update(
        CompareApiVersion(_v): CompareApiVersion,
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
        Json(organization_model): Json<organizations::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!(
            "UPDATE the entire Organization by id: {:?}, new name: {:?}",
            id, organization_model.name
        );

        let updated_organization: organizations::Model =
            OrganizationApi::update(app_state.db_conn_ref(), id, organization_model).await?;

        Ok(Json(updated_organization))
    }

    /// DELETE an Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request DELETE \
    /// http://localhost:4000/organizations/<id>
    pub async fn delete(
        CompareApiVersion(_v): CompareApiVersion,
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("DELETE Organization by id: {}", id);

        OrganizationApi::delete_by_id(app_state.db_conn_ref(), id).await?;
        Ok(Json(json!({"id": id})))
    }
}
