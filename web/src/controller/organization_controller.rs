use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::collections::HashMap;

pub struct OrganizationController {}

impl OrganizationController {
    pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
        let mut state_map = HashMap::new();

        state_map.insert(
            "database_connection".to_string(),
            app_state.database_connection.is_some().to_string(),
        );

        state_map.insert(
            "config".to_string(),
            app_state.config.database_uri().to_string(),
        );

        Json(state_map)
    }
}
