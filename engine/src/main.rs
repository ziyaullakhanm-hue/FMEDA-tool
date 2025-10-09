mod db;
mod models;
mod calc;
mod routes;
mod errors;

use axum::{Router, extract::State};
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::init_pool().await?;
    let app = Router::new()
        .merge(routes::router())
        .with_state(pool.clone());

    let addr: SocketAddr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".into()).parse()?;
    tracing::info!("Engine listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
