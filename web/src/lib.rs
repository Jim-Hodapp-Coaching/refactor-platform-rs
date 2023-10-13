pub use self::error::{Error, Result};
use axum::Server;
use service::AppState;
use std::net::SocketAddr;
use std::str::FromStr;

mod controller;
mod error;
mod router;

pub async fn init_server(app_state: AppState) -> Result<()> {
    // These will probably come from app_state.config (command line)
    let host = "127.0.0.1";
    let port = 3000;
    let server_url = format!("{host}:{port}");

    let addr = SocketAddr::from_str(&server_url).unwrap();

    println!("Server starting... listening for connections on http://{host}:{port}");

    // using unwrap() here as the app should panic if the server cannot start
    Server::bind(&addr)
        .serve(router::define_routes(app_state).into_make_service())
        .await
        .unwrap();

    Ok(())
}
