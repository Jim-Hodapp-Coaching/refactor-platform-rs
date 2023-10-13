use crate::AppState;
use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

use crate::controller::organization_controller::OrganizationController;

pub fn define_routes(app_state: AppState) -> Router {
    Router::new()
        .merge(organization_routes(app_state))
        .fallback_service(static_routes())
}

pub fn organization_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/organization", get(OrganizationController::index))
        .with_state(app_state)
}

// This will serve static files that we can use as a "fallback" for when the server panics
pub fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
