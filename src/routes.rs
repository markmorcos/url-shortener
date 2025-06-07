use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::ServeDir,
};

use crate::{
    handlers::{
        index_html::index_html, not_found::serve_404, redirect::redirect_url, shorten::shorten_url,
    },
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    let origin = AllowOrigin::exact("https://rdr.cx".parse().unwrap());

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/api", post(shorten_url))
        .route("/404", get(serve_404))
        .route("/", get(index_html))
        .nest_service("/static", ServeDir::new("static"))
        .route("/:id", get(redirect_url))
        .layer(cors)
        .with_state(state)
}
