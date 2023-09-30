use service::sea_orm::{ConnectOptions, Database, DbErr};
use tokio::time::Duration;

#[tokio::main]
async fn start() {
    // CREATE USER refactor_rs WITH PASSWORD 'password';
    // CREATE SCHEMA IF NOT EXISTS refactor_platform_rs;
    // SELECT schema_name FROM information_schema.schemata;
    // GRANT CREATE ON SCHEMA public TO refactor_rs;

    // cargo run --manifest-path ./migration/Cargo.toml -- up -u postgres://refactor_rs:password@localhost:5432/postgres -s refactor_platform_rs
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

    let db = Database::connect(opt)
        .await
        .expect("Database connection failed");

    db.ping().await.unwrap();

    println!("hello!");

    assert!(db.ping().await.is_ok());
    db.clone().close().await.unwrap();
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}

pub fn main() {
    start()
}
