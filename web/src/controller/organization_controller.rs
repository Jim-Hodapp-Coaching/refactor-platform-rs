use crate::{AppState, Error};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::organization;
use entity::organization::Entity as Organization;
use sea_orm::entity::EntityTrait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::DeleteResult;
use serde_json::json;
use entity_api::organization as OrganizationApi;

extern crate log;
use log::*;

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:3000/organizations
    pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
        let organizations = OrganizationApi::find_all(&app_state.database_connection.unwrap()).await;

        Json(organizations)
    }

    /// GET a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:3000/organizations/<id>
    pub async fn read(State(app_state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
        debug!("GET Organization by id: {}", id);

        let organization: Option<organization::Model> = organization::Entity::find_by_id(id)
            .one(&app_state.database_connection.unwrap())
            .await
            .unwrap_or_default();

        Json(organization)
    }

    /// CREATE a new Organization entity
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"name":"My New Organization"}' \
    /// http://localhost:3000/organizations
    pub async fn create(
        State(app_state): State<AppState>,
        Json(organization_json): Json<organization::Model>,
    ) -> impl IntoResponse {
        debug!("CREATE new Organization: {}", organization_json.name);

        let organization_active_model = organization::ActiveModel {
            id: NotSet,
            name: Set(organization_json.name),
        };

        let organization: organization::Model = organization_active_model
            .insert(&app_state.database_connection.unwrap())
            .await
            .unwrap_or_default();

        Json(organization)
    }

    /// UPDATE a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request PUT \
    /// http://localhost:3000/organizations/<id>\?name\=New_Organization_Name
    pub async fn update(
        State(app_state): State<AppState>,
        Path(id): Path<i32>,
        Query(organization_params): Query<organization::Model>,
    ) -> Result<Json<entity::organization::Model>, Error> {
        debug!(
            "UPDATE the entire Organization by id: {}, new name: {}",
            id, organization_params.name
        );

        let db = app_state.database_connection.as_ref().unwrap();

        let organization_to_update = organization::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap_or_default();

        let updated_organization = match organization_to_update {
            Some(org) => {
                let mut organization_am: organization::ActiveModel = org.into();
                organization_am.name = Set(organization_params.name);

                organization::Entity::update(organization_am)
                    .exec(db)
                    .await
                    .unwrap()
            }
            None => return Err(Error::EntityNotFound),
        };

        Ok(Json(updated_organization))
    }

    /// DELETE an Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request DELETE \
    /// http://localhost:3000/organizations/<id>
    pub async fn delete(
        State(app_state): State<AppState>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        debug!("DELETE Organization by id: {}", id);

        let res: DeleteResult = Organization::delete_by_id(id)
            .exec(&app_state.database_connection.unwrap())
            .await
            .unwrap();

        // TODO: temporary check while learning, return a DBErr instead
        assert_eq!(res.rows_affected, 1);

        Json(json!({"id": id}))
    }
}
