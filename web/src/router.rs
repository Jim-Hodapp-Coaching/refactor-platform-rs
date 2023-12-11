use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
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
        // TODO: Add an API versioning scheme and prefix all routes with it
        // See Router::nest() - https://docs.rs/axum/latest/axum/struct.Router.html#method.nest
        .route("/organizations", get(OrganizationController::index))
        .route("/organizations/hw", get(OrganizationController::read_hw))
        .route("/organizations/:id", get(OrganizationController::read))
        .route("/organizations", post(OrganizationController::create))
        .route("/organizations/:id", put(OrganizationController::update))
        .route("/organizations/:id", delete(OrganizationController::delete))
        .with_state(app_state)
}

// This will serve static files that we can use as a "fallback" for when the server panics
pub fn static_routes() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

#[cfg(test)]
mod organization_endpoints_tests {
    use super::*;
    use service::config::Config;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn read_test() -> anyhow::Result<()> {
        let app_state = AppState::new(Config::default());
        let router = define_routes(app_state);

        // TODO: set up mock DB instance?

        let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let client = reqwest::Client::new();

        let url = format!("http://{addr}/organizations/1");

        let response = client.get(url).send().await?.text().await?;
        assert_eq!(response, "Hello world");

        Ok(())
    }
}
