pub use self::error::{Error, Result};
use axum::Server;
use std::net::SocketAddr;
use std::str::FromStr;

mod error;
mod router;

pub async fn init_server() -> Result<()> {
    let host = "127.0.0.1";
    let port = 3000;
    let server_url = format!("{host}:{port}");

    let addr = SocketAddr::from_str(&server_url).unwrap();
    // using unwrap() here as the app should panic if the server cannot start
    Server::bind(&addr)
        .serve(router::define_routes().into_make_service())
        .await
        .unwrap();

    Ok(())
}
