use service::sea_orm::{entity::prelude::*, ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::time::Duration;

use entity::{coaching_relationship, organization};

//use refactor_platform_rs::config::Config;

#[tokio::main]
async fn start() {
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

    seed_database(db);
}

fn seed_database(db: DatabaseConnection) {
    let organization = organization::ActiveModel::from_json(json!({
        "name": "Jim Hodapp Coaching",
    }))?;

    assert_eq!(
        organization,
        organization::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set("Jim Hodapp Coaching".to_owned()),
        }
    );
}

pub fn main() {
    start()
}
