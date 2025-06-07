use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use redis::AsyncCommands;
use tracing::{info, warn};

use crate::state::AppState;

pub async fn redirect_url(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut con = state.redis_client.get_async_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
    let key = format!("u:{}", id);
    let url: Option<String> = con.get(&key).await.unwrap_or(None);

    match url {
        Some(url) => {
            info!("Redirecting to {}", url);
            Redirect::temporary(&url).into_response()
        }
        None => {
            warn!("Missing short URL for ID: {}", id);
            Redirect::permanent("/404").into_response()
        }
    }
}
