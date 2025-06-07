use axum::{extract::State, http::StatusCode, response::Json};
use base64::{engine::general_purpose, Engine};
use redis::AsyncCommands;
use tracing::{info, warn};

use crate::state::AppState;
use crate::types::{ShortenRequest, ShortenResponse};

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, StatusCode> {
    let sanitized_url = payload.url.trim().to_string();
    if sanitized_url.is_empty() || url::Url::parse(&sanitized_url).is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut con = state.redis_client.get_async_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://0.0.0.0:3000".into());

    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&sanitized_url);
    let reverse_key = format!("u:{}", encoded);
    if let Ok(Some(id)) = con.get::<_, Option<String>>(&reverse_key).await {
        return Ok(Json(ShortenResponse { short: format!("{}/{}", base_url, id) }));
    }

    let mut id;
    for attempt in 0..10 {
        id = nanoid::nanoid!(6);
        let key = format!("u:{}", id);
        let exists: bool = con.exists(&key).await.unwrap_or(true);
        if !exists {
            con.set(&key, &sanitized_url).await.unwrap_or(());
            con.set(&reverse_key, &id).await.unwrap_or(());
            info!("Created short URL: {} -> {}", id, sanitized_url);
            return Ok(Json(ShortenResponse { short: format!("{}/{}", base_url, id) }));
        }
        warn!("Collision on {}, attempt {}", id, attempt + 1);
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
