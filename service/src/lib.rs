use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::time::Duration;

//use refactor_platform_rs::config::Config;

pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new("postgres://refactor_rs:password@localhost:5432/postgres");
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

    Ok(db)
}
