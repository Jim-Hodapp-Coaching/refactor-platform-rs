use service::AppState;

pub mod error;
pub mod organization;

pub async fn seed_database(app_state: &AppState) {
    organization::seed_database(app_state).await;
}
