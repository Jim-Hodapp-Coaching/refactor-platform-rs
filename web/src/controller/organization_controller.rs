use crate::{AppState, Error};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::organization;
use entity_api::organization as OrganizationApi;
use serde_json::json;

extern crate log;
use log::*;

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations
    pub async fn index(State(app_state): State<AppState>) -> Result<impl IntoResponse, Error> {
        let organizations = OrganizationApi::find_all(app_state.db_conn_ref().unwrap()).await?;

        Ok(Json(organizations))
    }

    /// GET a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations/<id>
    pub async fn read(
        State(app_state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET Organization by id: {}", id);

        let organization: Option<organization::Model> =
            OrganizationApi::find_by_id(app_state.db_conn_ref().unwrap(), id).await?;

        Ok(Json(organization))
    }

    /// CREATE a new Organization entity
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"name":"My New Organization"}' \
    /// http://localhost:4000/organizations
    pub async fn create(
        State(app_state): State<AppState>,
        Json(organization_model): Json<organization::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("CREATE new Organization: {}", organization_model.name);

        let organization: organization::Model =
            OrganizationApi::create(app_state.db_conn_ref().unwrap(), organization_model).await?;

        debug!("Newly Created Organization: {:?}", &organization);

        Ok(Json(organization))
    }

    /// UPDATE a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request PUT  http://localhost:4000/organizations/<id> \
    /// --data '{"name":"My Updated Organization"}'
    pub async fn update(
        State(app_state): State<AppState>,
        Path(id): Path<i32>,
        Json(organization_model): Json<organization::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!(
            "UPDATE the entire Organization by id: {}, new name: {}",
            id, organization_model.name
        );

        let updated_organization: organization::Model =
            OrganizationApi::update(app_state.db_conn_ref().unwrap(), id, organization_model)
                .await?;

        Ok(Json(updated_organization))
    }

    /// DELETE an Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request DELETE \
    /// http://localhost:4000/organizations/<id>
    pub async fn delete(
        State(app_state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("DELETE Organization by id: {}", id);

        OrganizationApi::delete_by_id(app_state.db_conn_ref().unwrap(), id).await?;
        Ok(Json(json!({"id": id})))
    }
}
