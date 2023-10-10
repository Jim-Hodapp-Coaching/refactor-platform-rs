use serde_json::json;
use service::sea_orm::{
    entity::prelude::*, query::*, ActiveValue, ConnectOptions, Database, DatabaseConnection, DbErr,
};
use tokio::time::Duration;

use entity::{coaching_relationship, organization, user};

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

    seed_database(db).await;
}

async fn seed_database(db: DatabaseConnection) {
    let organization = organization::ActiveModel::from_json(json!({
        "name": "Jim Hodapp Coaching",
    }))
    .unwrap();

    assert_eq!(
        organization,
        organization::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set("Jim Hodapp Coaching".to_owned()),
        }
    );

    let persisted_org: organization::Model = organization.insert(&db).await.unwrap();

    let queried_org: Option<organization::Model> =
        organization::Entity::find_by_id(persisted_org.id)
            .one(&db)
            .await
            .unwrap();

    println!("queried_org: {:?}", queried_org);

    let caleb = user::ActiveModel::from_json(json!({
        "email": "calebbourg2@gmail.com",
        "first_name": "Caleb",
        "last_name": "Bourg"
    }))
    .unwrap();

    let persisted_caleb = caleb.insert(&db).await.unwrap();

    let queried_caleb: Option<user::Model> = user::Entity::find_by_id(persisted_caleb.id)
        .one(&db)
        .await
        .unwrap();

    println!("queried_caleb: {:?}", queried_caleb);

    let jim = user::ActiveModel::from_json(json!({
        "email": "jim@jimhodappcoaching.com",
        "first_name": "Jim",
        "last_name": "Hodapp"
    }))
    .unwrap();

    let persisted_jim = jim.insert(&db).await.unwrap();

    let queried_jim: Option<user::Model> = user::Entity::find_by_id(persisted_jim.id)
        .one(&db)
        .await
        .unwrap();

    println!("queried_jim: {:?}", queried_jim);
}

pub fn main() {
    start()
}
