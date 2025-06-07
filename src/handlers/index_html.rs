use axum::{http::StatusCode, response::Html};
use tokio::fs;

pub async fn index_html() -> Result<Html<String>, (StatusCode, String)> {
    match fs::read_to_string("static/index.html").await {
        Ok(contents) => Ok(Html(contents)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
