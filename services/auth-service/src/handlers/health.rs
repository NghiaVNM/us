//! Health check handler.

use axum::Json;
use serde_json::{json, Value};

/// Health check endpoint.
pub async fn health_check() -> Json<Value> {
  Json(json!({
    "status": "ok",
    "service": "auth-service",
  }))
}