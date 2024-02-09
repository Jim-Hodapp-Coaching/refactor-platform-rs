use sea_orm::DatabaseConnection;

pub mod error;
pub mod organization;
pub mod user;

pub async fn seed_database(db: &DatabaseConnection) {
    organization::seed_database(db).await;
    user::seed_database(db).await;
}
