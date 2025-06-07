use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};
use tokio::fs;

pub async fn serve_404() -> Response {
    match fs::read_to_string("static/404.html").await {
        Ok(content) => (StatusCode::NOT_FOUND, Html(content)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Html("<h1>404 Not Found</h1>".to_string())).into_response(),
    }
}
