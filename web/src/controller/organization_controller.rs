use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use entity::organization;
use sea_orm::entity::EntityTrait;

pub struct OrganizationController {}

impl OrganizationController {
    pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
        let organizations = organization::Entity::find()
            .all(&app_state.database_connection.unwrap())
            .await
            .unwrap_or(vec![]);

        Json(organizations)
    }
}
