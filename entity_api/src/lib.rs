use sea_orm::DatabaseConnection;

pub mod error;
pub mod organization;

pub async fn seed_database(db: &DatabaseConnection) {
    organization::seed_database(db).await;
}
