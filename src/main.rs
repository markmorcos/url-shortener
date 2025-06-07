mod handlers;
mod routes;
mod state;
mod types;

use crate::{routes::app, state::AppState};
use axum::serve;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let redis_client = redis::Client::open(redis_url)?;
    let mut con = redis_client.get_async_connection().await?;
    redis::cmd("PING").query_async::<_, ()>(&mut con).await?;

    let state = AppState {
        redis_client: Arc::new(redis_client),
    };

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app(state).into_make_service()).await?;

    Ok(())
}
