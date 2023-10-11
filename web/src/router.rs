use axum::{
    response::Html,
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub fn define_routes() -> Router {
    Router::new()
        .merge(base_routes())
        .fallback_service(static_routes())
}

pub fn base_routes() -> Router {
    Router::new().route(
        "/",
        get(|| async { Html("<p>this handler will be in a controller</p>") }),
    )
}

// This will serve static files that we can use as a "fallback" for when the server panics
pub fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
