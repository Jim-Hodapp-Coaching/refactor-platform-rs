use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum_login::login_required;
use entity_api::user::Backend;
use tower_http::services::ServeDir;

use crate::controller::{
    organization_controller::OrganizationController, user_session_controller::UserSessionController,
};

pub fn define_routes(app_state: AppState) -> Router {
    Router::new()
        .merge(organization_routes(app_state))
        .merge(session_routes())
        .merge(protected_routes())
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
        .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(app_state)
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/protected", get(UserSessionController::protected))
        .route("/logout", get(UserSessionController::logout))
        .route_layer(login_required!(Backend, login_url = "/login"))
}

pub fn session_routes() -> Router {
    Router::new().route("/login", post(UserSessionController::login))
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
    use anyhow::Ok;
    use axum_login::{
        tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
        AuthManagerLayerBuilder,
    };
    use chrono::Utc;
    use entity::{organizations, users};
    use entity_api::user::Backend;
    use log::{debug, LevelFilter};
    use password_auth::generate_hash;
    use reqwest::Url;
    use sea_orm::{
        prelude::Uuid, DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult,
    };
    use serde_json::json;
    use service::{config::Config, logging::Logger};
    use std::{net::SocketAddr, sync::Arc, sync::Once};
    use time::Duration;
    use tokio::net::TcpListener;

    static INIT: Once = Once::new();

    // Enable and call this at the start of a particular test to turn on DEBUG
    // level logging output used to debug a new or existing test.
    // Change to Trace to see all output.
    fn enable_test_logging(config: &mut Config) {
        INIT.call_once(|| {
            config.log_level_filter = LevelFilter::Debug;
            Logger::init_logger(&config);
        });
    }

    // A test wrapper that sets up both an http server instance with the router backend
    // endpoints and a Reqwest-based http client used to call the backend server.
    //
    // Adapted from: https://blog.sedrik.se/posts/secure-axum/
    #[derive(Clone, Debug)]
    pub struct TestClientServer {
        pub client: reqwest::Client,
        addr: String,
    }

    impl TestClientServer {
        pub async fn new(router: Router, db: &Arc<DatabaseConnection>) -> anyhow::Result<Self> {
            let session_store = MemoryStore::default();

            let session_layer = SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::days(1)));

            // Auth service
            let backend = Backend::new(db);
            let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

            let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;
            let addr = listener.local_addr()?;

            tokio::spawn(async move {
                axum::serve(listener, router.layer(auth_layer).into_make_service())
                    .await
                    .unwrap();
            });

            let client = reqwest::Client::builder().cookie_store(true).build()?;

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

        /// Logs into a new AuthSession with a session cookie. The cookie will be
        /// cached by the client Reqwest instance because we constructed the client
        /// with it turned on (i.e. `cookie_store(true)`).
        ///
        /// This is meant to be reused by all tests that sit behind a protected route.
        pub async fn login(&mut self, user: &users::Model) -> anyhow::Result<()> {
            let creds = [("email", "test@domain.com"), ("password", "password2")];
            let response = self
                .client
                .post(self.url("/login").unwrap())
                .form(&creds)
                .send()
                .await?;

            let response_text = response.text().await?;

            debug!("response_text: {:?}", response_text);

            assert_eq!(
                response_text,
                json!({
                    "display_name": user.display_name,
                    "email": user.email,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                })
                .to_string()
            );

            Ok(())
        }

        /// Creates a test user::Model entity instance that can be used by tests to
        /// log in to the /login endpoint and create a valid AuthSession.
        pub fn get_user() -> anyhow::Result<users::Model> {
            let now = Utc::now();
            Ok(users::Model {
                id: 1,
                email: "test@domain.com".to_string(),
                first_name: Some("test".to_string()),
                last_name: Some("login".to_string()),
                display_name: Some("test login".to_string()),
                password: generate_hash("password2").to_owned(),
                github_username: None,
                github_profile_url: None,
                created_at: Some(now.into()),
                updated_at: Some(now.into()),
                external_id: Uuid::new_v4(),
            })
        }
    }

    // Purpose: adds an Organization instance to a mock DB and tests the API to successfully
    // retrieve it by a specific ID and as expected and valid JSON.
    #[tokio::test]
    async fn read_returns_expected_json_for_specified_organization() -> anyhow::Result<()> {
        let mut config = Config::default();
        enable_test_logging(&mut config);

        let user = TestClientServer::get_user().expect("Creating a new test user failed");

        let organization_results = [vec![organizations::Model {
            id: 1,
            name: Some("Organization One".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        }]];

        let db = Arc::new(
            MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![user.clone()]])
                .append_query_results(organization_results.clone())
                .into_connection(),
        );

        let app_state = AppState::new(config, &db);

        let mut test_client_server = TestClientServer::new(define_routes(app_state), &db)
            .await
            .unwrap();

        let response = test_client_server.login(&user).await?;

        assert_eq!(response, ()); // Make sure we get a 200 OK response

        let response = test_client_server
            .client
            .get(test_client_server.url("/organizations/1").unwrap())
            .send()
            .await?;

        let response_text = response.text().await?;

        let organization1 = &organization_results[0][0];
        assert_eq!(response_text, json!(organization1).to_string());

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests the API to successfully
    // retrieve all of them as expected and valid JSON without specifying any particular ID.
    #[tokio::test]
    async fn read_returns_all_organizations() -> anyhow::Result<()> {
        let mut config = Config::default();
        enable_test_logging(&mut config);

        let user = TestClientServer::get_user().expect("Creating a new test user failed");
        let user_results1 = [vec![user.clone()]];

        // Note: for entity_api::organization::find_all() to be able to return
        // the correct query_results for the assert_eq!() below, they must all
        // be grouped together in the same inner vector.
        let organizations = [vec![
            organizations::Model {
                id: 1,
                name: Some("Organization One".to_owned()),
                created_at: None,
                updated_at: None,
                logo: None,
                external_id: Uuid::new_v4(),
            },
            organizations::Model {
                id: 2,
                name: Some("Organization Two".to_owned()),
                created_at: None,
                updated_at: None,
                logo: None,
                external_id: Uuid::new_v4(),
            },
            organizations::Model {
                id: 3,
                name: Some("Organization Three".to_owned()),
                created_at: None,
                updated_at: None,
                logo: None,
                external_id: Uuid::new_v4(),
            },
        ]];

        let db = Arc::new(
            MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results(user_results1.clone())
                .append_query_results(user_results1.clone())
                .append_query_results(organizations.clone())
                .into_connection(),
        );

        let app_state = AppState::new(config, &db);

        let mut test_client_server = TestClientServer::new(define_routes(app_state), &db)
            .await
            .unwrap();

        let response = test_client_server.login(&user).await?;
        assert_eq!(response, ());

        let response = test_client_server
            .client
            .get(test_client_server.url("/organizations").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let organization1 = &organizations[0][0];
        let organization2 = &organizations[0][1];
        let organization3 = &organizations[0][2];

        assert_eq!(
            response,
            json!([organization1, organization2, organization3]).to_string()
        );

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests that calling
    // the appropriate endpoint deletes instances specified by distinct IDs.
    #[tokio::test]
    async fn delete_an_organization_specified_by_id() -> anyhow::Result<()> {
        let mut config = Config::default();
        enable_test_logging(&mut config);

        let user = TestClientServer::get_user().expect("Creating a new test user failed");
        let user_results1 = [vec![user.clone()]];

        let organization_results1 = [vec![organizations::Model {
            id: 2,
            name: Some("Organization Two".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        }]];
        let organization_results2 = [vec![organizations::Model {
            id: 3,
            name: Some("Organization Three".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        }]];

        let exec_results1 = [MockExecResult {
            last_insert_id: 2,
            rows_affected: 1,
        }];

        let exec_results2 = [MockExecResult {
            last_insert_id: 3,
            rows_affected: 1,
        }];

        let db = Arc::new(
            MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results(user_results1.clone()) // For the initial login auth check
                .append_query_results(user_results1.clone()) // For the AuthSession check done with the next endpoint call
                .append_query_results(organization_results1.clone()) // For comparing the first organization query results with
                .append_exec_results(exec_results1) // For comparing the first organization query execution results with
                .append_query_results(user_results1.clone()) // For the AuthSession check done with the next endpoint call
                .append_query_results(organization_results1.clone()) // For compare the second organization query results with
                .append_exec_results(exec_results2) // For comparing the second organization query execution results with
                .into_connection(),
        );

        let app_state = AppState::new(config, &db);

        let mut test_client_server = TestClientServer::new(define_routes(app_state), &db)
            .await
            .unwrap();

        let response = test_client_server.login(&user).await?;
        assert_eq!(response, ());

        {
            let response = test_client_server
                .client
                .delete(test_client_server.url("/organizations/2").unwrap())
                .send()
                .await?
                .text()
                .await?;

            let organization2 = &organization_results1[0][0];

            assert_eq!(
                response,
                json!({
                    "id": organization2.id,
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

            let organization3 = &organization_results2[0][0];

            assert_eq!(
                response,
                json!({
                    "id": organization3.id,
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
        let mut config = Config::default();
        enable_test_logging(&mut config);

        let user = TestClientServer::get_user().expect("Creating a new test user failed");
        let user_results1 = [vec![user.clone()]];

        let organization_results1 = [vec![organizations::Model {
            id: 5,
            name: Some("New Organization Five".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        }]];

        let organization_results2 = [vec![organizations::Model {
            id: 6,
            name: Some("Second Organization Six".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        }]];

        let exec_results1 = [MockExecResult {
            last_insert_id: 5,
            rows_affected: 1,
        }];

        let exec_results2 = [MockExecResult {
            last_insert_id: 6,
            rows_affected: 1,
        }];

        let db = Arc::new(
            MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results(user_results1.clone())
                .append_query_results(user_results1.clone())
                .append_query_results(organization_results1.clone())
                .append_exec_results(exec_results1)
                .append_query_results(user_results1.clone())
                .append_query_results(organization_results2.clone())
                .append_exec_results(exec_results2)
                .into_connection(),
        );

        let app_state = AppState::new(config, &db);

        let mut test_client_server = TestClientServer::new(define_routes(app_state), &db)
            .await
            .unwrap();

        let response = test_client_server.login(&user).await?;
        assert_eq!(response, ());

        {
            let organization5 = &organization_results1[0][0];

            let response = test_client_server
                .client
                .post(test_client_server.url("/organizations").unwrap())
                .json(&organization5)
                .send()
                .await?
                .text()
                .await?;

            assert_eq!(response, json!(organization5).to_string());
        }

        {
            let organization6 = &organization_results2[0][0];

            let response = test_client_server
                .client
                .post(test_client_server.url("/organizations").unwrap())
                .json(&organization6)
                .send()
                .await?
                .text()
                .await?;

            assert_eq!(response, json!(organization6).to_string());
        }

        Ok(())
    }

    // Purpose: adds multiple Organization instances to a mock DB and tests that calling
    // the appropriate endpoint updates an instance specified by an ID.
    #[tokio::test]
    async fn update_an_organization_specified_by_id() -> anyhow::Result<()> {
        let mut config = Config::default();
        enable_test_logging(&mut config);

        let user = TestClientServer::get_user().expect("Creating a new test user failed");
        let user_results1 = [vec![user.clone()]];

        let organizations = [
            vec![organizations::Model {
                id: 2,
                name: Some("Organization Two".to_owned()),
                created_at: None,
                updated_at: None,
                logo: None,
                external_id: Uuid::new_v4(),
            }],
            vec![organizations::Model {
                id: 2,
                name: Some("Updated Organization Two".to_owned()),
                created_at: None,
                updated_at: None,
                logo: None,
                external_id: Uuid::new_v4(),
            }],
        ];

        let exec_results = [MockExecResult {
            last_insert_id: 2,
            rows_affected: 1,
        }];

        let db = Arc::new(
            MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results(user_results1.clone())
                .append_query_results(user_results1.clone())
                .append_query_results(organizations.clone())
                .append_exec_results(exec_results)
                .into_connection(),
        );

        let app_state = AppState::new(config, &db);

        let mut test_client_server = TestClientServer::new(define_routes(app_state), &db)
            .await
            .unwrap();

        let response = test_client_server.login(&user).await?;
        assert_eq!(response, ());

        let updated_organization2 = organizations::Model {
            id: 2,
            name: Some("Updated Organization Two".to_owned()),
            created_at: None,
            updated_at: None,
            logo: None,
            external_id: Uuid::new_v4(),
        };

        let response = test_client_server
            .client
            .put(test_client_server.url("/organizations/2").unwrap())
            .json(&updated_organization2)
            .send()
            .await?
            .text()
            .await?;

        assert_eq!(response, json!(updated_organization2).to_string());

        Ok(())
    }
}
