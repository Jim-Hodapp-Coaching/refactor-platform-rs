use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::services::ServeDir;

use crate::controller::{organization_controller::OrganizationController, user_session_controller::UserSessionController};

pub fn define_routes(app_state: AppState) -> Router {
    root_router()
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

        .route("/login/password", post(UserSessionController::login))
        .with_state(app_state)
}

pub fn root_router() -> Router {
    Router::new().route("/", get(UserSessionController::index))
}

// TODO: rename to session_routes or user_session_routes?
// pub fn service_routes() -> Router {
//     Router::new()
//         // TODO: Add an API versioning scheme and prefix all routes with it
//         // See Router::nest() - https://docs.rs/axum/latest/axum/struct.Router.html#method.nest
//         .route("/login/password", post(UserSessionController::login))
// }

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
    use log::LevelFilter;
    use reqwest::Url;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serde_json::json;
    use service::{config::Config, logging::Logger};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    // Enable and call this at the start of a particular test to turn on TRACE
    // level logging output used to debug a new or existing test.
    fn _enable_test_logging(config: &mut Config) {
        config.log_level_filter = LevelFilter::Trace;
        Logger::init_logger(&config);
    }

    // A test wrapper that sets up both an http server instance with the router backend
    // endpoints and a Reqwest-based http client used to call the backend server.
    //
    // Adapted from: https://blog.sedrik.se/posts/secure-axum/
    #[derive(Clone)]
    pub struct TestClientServer {
        pub client: reqwest::Client,
        addr: String,
    }

    impl TestClientServer {
        pub async fn new(router: Router) -> anyhow::Result<Self> {
            let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;
            let addr = listener.local_addr()?;

            tokio::spawn(async move {
                axum::serve(listener, router).await.unwrap();
            });

            let client = reqwest::Client::new();

            Ok(Self {
                client,
                addr: addr.to_string(),
            })
        }

        pub fn url<S: AsRef<str>>(&self, path: S) -> anyhow::Result<String> {
            let base = Url::parse(format!("http://{}", self.addr).as_ref())?;
            let url = base.join(path.as_ref())?;
            Ok(url.as_str().to_string())
        }
    }

    // Purpose: adds an Organization instance to a mock DB and tests the API to successfully
    // retrieve it by a specific ID and as expected and valid JSON.
    #[tokio::test]
    async fn read_returns_expected_json_for_specified_organization() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        let organizations = [vec![organization::Model {
            id: 1,
            name: "Organization One".to_owned(),
        }]];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .into_connection();

        app_state.set_db_conn(db);

        let test_client_server = TestClientServer::new(define_routes(app_state))
            .await
            .unwrap();

        let response = test_client_server
            .client
            .get(test_client_server.url("/organizations/1").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let organization_model1 = &organizations[0][0];
        assert_eq!(response, json!(organization_model1).to_string());

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

        let test_client_server = TestClientServer::new(define_routes(app_state))
            .await
            .unwrap();

        let response = test_client_server
            .client
            .get(test_client_server.url("/organizations").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let organization_model1 = &organizations[0][0];
        let organization_model2 = &organizations[0][1];
        let organization_model3 = &organizations[0][2];

        assert_eq!(
            response,
            json!([
                organization_model1,
                organization_model2,
                organization_model3
            ])
            .to_string()
        );

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests that calling
    // the appropriate endpoint deletes instances specified by distinct IDs.
    #[tokio::test]
    async fn delete_an_organization_specified_by_id() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        let organizations = [
            vec![organization::Model {
                id: 2,
                name: "Organization Two".to_owned(),
            }],
            vec![organization::Model {
                id: 3,
                name: "Organization Three".to_owned(),
            }],
        ];

        let exec_results = [
            MockExecResult {
                last_insert_id: 2,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 3,
                rows_affected: 1,
            },
        ];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .append_exec_results(exec_results)
            .into_connection();

        app_state.set_db_conn(db);

        let test_client_server = TestClientServer::new(define_routes(app_state))
            .await
            .unwrap();
        {
            let response = test_client_server
                .client
                .delete(test_client_server.url("/organizations/2").unwrap())
                .send()
                .await?
                .text()
                .await?;

            let organization_model2 = &organizations[0][0];

            assert_eq!(
                response,
                json!({
                    "id": organization_model2.id,
                })
                .to_string()
            );
        }

        {
            let response = test_client_server
                .client
                .delete(test_client_server.url("/organizations/3").unwrap())
                .send()
                .await?
                .text()
                .await?;

            let organization_model3 = &organizations[1][0];

            assert_eq!(
                response,
                json!({
                    "id": organization_model3.id,
                })
                .to_string()
            );
        }

        Ok(())
    }

    // Purpose: creates multiple new Organization instances to a mock DB by calling
    // the post endpoint supplying the appropriate instance as a JSON payload.
    #[tokio::test]
    async fn create_new_organizations_successfully() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        let organizations = [
            vec![organization::Model {
                id: 5,
                name: "New Organization Five".to_owned(),
            }],
            vec![organization::Model {
                id: 6,
                name: "Second Organization Six".to_owned(),
            }],
        ];

        let exec_results = [
            MockExecResult {
                last_insert_id: 5,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
        ];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .append_exec_results(exec_results)
            .into_connection();

        app_state.set_db_conn(db);

        let test_client_server = TestClientServer::new(define_routes(app_state))
            .await
            .unwrap();
        {
            let organization_model5 = &organizations[0][0];

            let response = test_client_server
                .client
                .post(test_client_server.url("/organizations").unwrap())
                .json(&organization_model5)
                .send()
                .await?
                .text()
                .await?;

            assert_eq!(response, json!(organization_model5).to_string());
        }

        {
            let organization_model6 = &organizations[1][0];

            let response = test_client_server
                .client
                .post(test_client_server.url("/organizations").unwrap())
                .json(&organization_model6)
                .send()
                .await?
                .text()
                .await?;

            assert_eq!(response, json!(organization_model6).to_string());
        }

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests that calling
    // the appropriate endpoint updates an instance specified by an ID.
    #[tokio::test]
    async fn update_an_organization_specified_by_id() -> anyhow::Result<()> {
        let mut app_state = AppState::new(Config::default());

        let organizations = [
            vec![organization::Model {
                id: 2,
                name: "Organization Two".to_owned(),
            }],
            vec![organization::Model {
                id: 2,
                name: "Updated Organization Two".to_owned(),
            }],
        ];

        let exec_results = [MockExecResult {
            last_insert_id: 2,
            rows_affected: 1,
        }];

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(organizations.clone())
            .append_exec_results(exec_results)
            .into_connection();

        app_state.set_db_conn(db);

        let test_client_server = TestClientServer::new(define_routes(app_state))
            .await
            .unwrap();

        let updated_organization_model2 = organization::Model {
            id: 2,
            name: "Updated Organization Two".to_owned(),
        };

        let response = test_client_server
            .client
            .put(test_client_server.url("/organizations/2").unwrap())
            .json(&updated_organization_model2)
            .send()
            .await?
            .text()
            .await?;

        assert_eq!(response, json!(updated_organization_model2).to_string());

        Ok(())
    }
}
