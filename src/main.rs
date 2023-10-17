use service::{config::Config, AppState};

extern crate simplelog;

#[tokio::main]
async fn main() {
    let config = get_config();
    let log_level = match config.trace_level {
        0 => simplelog::LevelFilter::Warn,
        1 => simplelog::LevelFilter::Debug,
        2 => simplelog::LevelFilter::Info,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Trace,
    };

    simplelog::TermLogger::init(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Failed to start simplelog");

    simplelog::info!("<b>Starting up...</b>.");

    let mut app_state = AppState::new(config);
    app_state = service::init_database(app_state).await.unwrap();

    entity_api::seed_database(app_state.database_connection.as_ref().unwrap()).await;

    web::init_server(app_state).await.unwrap();
}

fn get_config() -> Config {
    Config::new()
}
