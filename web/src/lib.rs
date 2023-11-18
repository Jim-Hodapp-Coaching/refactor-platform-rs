pub use self::error::{Error, Result};
use axum::Server;
use service::AppState;
use std::net::SocketAddr;
use std::str::FromStr;

extern crate log;

use log::*;

mod controller;
mod error;
mod router;

pub async fn init_server(app_state: AppState) -> Result<()> {
    // These will probably come from app_state.config (command line)
    let host = app_state.config.interface.as_ref().unwrap();
    let port = app_state.config.port;
    let server_url = format!("{host}:{port}");

    let listen_addr = SocketAddr::from_str(&server_url).unwrap();

    info!("Server starting... listening for connections on http://{host}:{port}");

    // using unwrap() here as the app should panic if the server cannot start
    Server::bind(&listen_addr)
        .serve(router::define_routes(app_state).into_make_service())
        .await
        .unwrap();

    Ok(())
}
