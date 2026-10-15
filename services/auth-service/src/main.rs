//! Auth service - User authentication and session management.

mod config;
mod error;
mod handlers;
mod middleware;
mod models;

use axum::{
  routing::get,
  Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Initialize tracing
  us_service_kit::tracing::init_tracing();

  // Load configuration
  let config = Config::load()?;
  tracing::info!("Loaded configuration: {:?}", config);

  // Build router
  let app = Router::new()
    .route("/health", get(handlers::health::health_check))
    .route("/api/v1/me", get(handlers::user::get_current_user))
    .layer(TraceLayer::new_for_http())
    .with_state(config.clone());

  // Start server
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
  tracing::info!("Auth service listening on {}", addr);
  
  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}