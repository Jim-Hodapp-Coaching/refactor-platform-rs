// mod config;

// use config::*;
#[tokio::main]
async fn main() {
    // let app_config = get_config();
    // if let Some(uri) = app_config.uri.as_deref() {
    //     println!("DB URI: {uri}");
    // }

    entity_api::init_database().await;
    web::init_server().await.unwrap();
}
