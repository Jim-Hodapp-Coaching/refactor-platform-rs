use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, PostgresStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use entity_api::user::Backend;

pub use self::error::{Error, Result};
use log::*;
use service::AppState;
use std::net::SocketAddr;
use std::str::FromStr;
use time::Duration;
use tokio::net::TcpListener;

mod controller;
mod error;
mod router;

pub async fn init_server(app_state: AppState) -> Result<()> {
    // Session layer
    let session_store = PostgresStore::new(
        app_state
            .db_conn_ref()
            .get_postgres_connection_pool()
            .to_owned(),
    );
    // TODO: this put the created session under the tower_sessions/sessions in the DB, do we want this to
    // be under refactor_platform_rs/user_sessions or something?
    // See the following for setting the schema/table name pair:
    // https://docs.rs/tower-sessions/latest/tower_sessions/struct.PostgresStore.html
    session_store.migrate().await.unwrap();

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service
    let backend = Backend::new(app_state.db_conn_ref());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // These will probably come from app_state.config (command line)
    let host = app_state.config.interface.as_ref().unwrap();
    let port = app_state.config.port;
    let server_url = format!("{host}:{port}");

    let listen_addr = SocketAddr::from_str(&server_url).unwrap();

    info!("Server starting... listening for connections on http://{host}:{port}");

    let listener = TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(
        listener,
        router::define_routes(app_state)
            .layer(auth_layer)
            .into_make_service(),
    )
    .await
    .unwrap();

    let _res = deletion_task.await.unwrap();

    Ok(())
}
