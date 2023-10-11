// mod config;

// use config::*;
#[tokio::main]
async fn main() {
    // let app_config = get_config();
    // if let Some(uri) = app_config.uri.as_deref() {
    //     println!("DB URI: {uri}");
    // }

    // Returning a DatabaseConnection here in case we need to pass it
    // into the axum Router context at some point to make it available to
    // route handlers (controllers)
    let _db = service::init_database().await.unwrap();
    web::init_server().await.unwrap();
}
