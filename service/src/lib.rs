use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::time::Duration;

pub mod config;

pub async fn init_database(mut app_state: AppState) -> Result<AppState, DbErr> {
    let mut opt = ConnectOptions::new::<&str>(app_state.config.database_uri().as_ref());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("refactor_platform_rs"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;

    app_state.database_connection = Arc::new(Some(db));

    Ok(app_state)
}

// Needs to implement Clone to be able to be passed into Router as State
#[derive(Clone)]
pub struct AppState {
    pub database_connection: Arc<Option<DatabaseConnection>>,
    pub config: Config,
}

impl AppState {
    pub fn new(app_config: Config) -> Self {
        Self {
            database_connection: Arc::new(None),
            config: app_config,
        }
    }

    pub fn db_conn_ref(&self) -> Option<&DatabaseConnection> {
        self.database_connection.as_ref().as_ref()
    }
}
