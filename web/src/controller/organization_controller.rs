use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::organization;
use entity::organization::Entity as Organization;
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::ActiveModelTrait;
use sea_orm::DeleteResult;
use sea_orm::entity::EntityTrait;
use serde_json::json;

extern crate log;
use log::*;

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:3000/organizations
    pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
        let organizations = organization::Entity::find()
            .all(&app_state.database_connection.unwrap())
            .await
            .unwrap_or(vec![]);

        Json(organizations)
    }

    /// CREATE a new Organization entity
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"name":"My New Organization"}' \
    /// http://localhost:3000/organizations
    pub async fn create(State(app_state): State<AppState>, Json(organization_json): Json<organization::Model>) -> impl IntoResponse {
        debug!("CREATE new Organization: {}", organization_json.name);

        let organization_active_model = organization::ActiveModel {
            id: NotSet,
            name: Set(organization_json.name),
        };

        let organization: organization::Model = organization_active_model.insert(&app_state.database_connection.unwrap())
            .await
            .unwrap();

        Json(organization)
    }

    /// DELETE an Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request DELETE \
    /// http://localhost:3000/organizations/<id>
    pub async fn delete(State(app_state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
        debug!("DELETE Organization by id: {}", id);

        let res: DeleteResult = Organization::delete_by_id(id).exec(&app_state.database_connection.unwrap())
            .await
            .unwrap();

        // TODO: temporary check while learning, return a DBErr instead
        assert_eq!(res.rows_affected, 1);

        Json(json!({"id": id}))
    }
}
