use axum::{routing::{get, post}, Router};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{
    handlers::{not_found::serve_404, redirect::redirect_url, shorten::shorten_url},
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/api", post(shorten_url))
        .route("/404", get(serve_404))
        .route("/:id", get(redirect_url))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
