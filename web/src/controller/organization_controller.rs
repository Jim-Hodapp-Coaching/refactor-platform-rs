use crate::{custom_extractors::CheckApiVersion, AppState, Error};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::{organizations, Id};
use entity_api::organization as OrganizationApi;
use serde_json::json;

use log::*;

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations
    pub async fn index(
        CheckApiVersion(_v): CheckApiVersion,
        State(app_state): State<AppState>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET all Organizations");
        let organizations = OrganizationApi::find_all(app_state.db_conn_ref()).await?;

        debug!("Found Organizations: {:?}", organizations);

        Ok(Json(organizations))
    }

    /// GET a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations/<id>
    pub async fn read(
        CheckApiVersion(_v): CheckApiVersion,
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
        CheckApiVersion(_v): CheckApiVersion,
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
        CheckApiVersion(_v): CheckApiVersion,
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
        CheckApiVersion(_v): CheckApiVersion,
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("DELETE Organization by id: {}", id);

        OrganizationApi::delete_by_id(app_state.db_conn_ref(), id).await?;
        Ok(Json(json!({"id": id})))
    }
}
