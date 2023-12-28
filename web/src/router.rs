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
// We need to gate seaORM's mock feature behind conditional compilation because
// the feature removes the Clone trait implementation from seaORM's DatabaseConnection.
// see https://github.com/SeaQL/sea-orm/issues/830
#[cfg(feature = "mock")]
mod organization_endpoints_tests {
    use super::*;
    use entity::organization;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serde_json::json;
    use service::config::Config;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    // Purpose: adds an Organization instance to a mock DB and tests the API to successfully
    // retrieve it by a specific ID and as expected and valid JSON.
    #[tokio::test]
    async fn read_returns_expected_json_for_specified_organization() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        let organizations = vec![vec![organization::Model {
            id: 1,
            name: "Organization One".to_owned(),
        }]];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .into_connection();

        app_state.set_db_conn(db);
        let router = define_routes(app_state);

        let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let client = reqwest::Client::new();

        let url = format!("http://{addr}/organizations/1");

        let response = client.get(url).send().await?.text().await?;
        assert_eq!(response, json!(organizations[0][0]).to_string());

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests the API to successfully
    // retrieve all of them as expected and valid JSON without specifying any particular ID.
    #[tokio::test]
    async fn read_returns_all_organizations() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        // Note: for entity_api::organization::find_all() to be able to return
        // the correct query_results for the assert_eq!() below, they must all
        // be grouped together in the same inner vector.
        let organizations = [vec![
            organization::Model {
                id: 1,
                name: "Organization One".to_owned(),
            },
            organization::Model {
                id: 2,
                name: "Organization Two".to_owned(),
            },
            organization::Model {
                id: 3,
                name: "Organization Three".to_owned(),
            },
        ]];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .into_connection();

        app_state.set_db_conn(db);
        let router = define_routes(app_state);

        let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        let client = reqwest::Client::new();

        let url = format!("http://{addr}/organizations");

        let response = client.get(url).send().await?.text().await?;

        let organization_model1 = &organizations[0][0];
        let organization_model2 = &organizations[0][1];
        let organization_model3 = &organizations[0][2];

        assert_eq!(
            response,
            json!([{
                 "id": organization_model1.id,
                 "name": organization_model1.name,
            },{
                 "id": organization_model2.id,
                 "name": organization_model2.name,
            },{
                 "id": organization_model3.id,
                 "name": organization_model3.name,
            }])
            .to_string()
        );

        Ok(())
    }
}
