use log::info;
use service::{config::Config, logging::Logger, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config::new();
    Logger::init_logger(&config as &Config);

    info!("Seeding Database...");

    let db = Arc::new(service::init_database(config.database_uri()).await.unwrap());

    let app_state = AppState::new(config, &db);

    entity_api::seed_database(app_state.db_conn_ref()).await;
}
