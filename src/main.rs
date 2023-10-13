use service::{config::Config, AppState};

#[tokio::main]
async fn main() {
    let config = get_config();
    let mut app_state = AppState::new(config);
    app_state = service::init_database(app_state).await.unwrap();

    entity_api::seed_database(app_state.database_connection.as_ref().unwrap());

    web::init_server(app_state).await.unwrap();
}

fn get_config() -> Config {
    let config = Config::new();
    // handle command line args and return service::config::Config struct
    config.set_database_uri("postgres://refactor_rs:password@localhost:5432/postgres".to_string())
}
