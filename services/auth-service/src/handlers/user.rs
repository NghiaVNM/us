//! User handlers.

use axum::Json;
use serde_json::{json, Value};

use crate::error::Result;

/// Get current user.
///
/// TODO: Extract user from session middleware.
pub async fn get_current_user() -> Result<Json<Value>> {
  // Placeholder - will implement with middleware
  Ok(Json(json!({
    "user": {
      "id": "placeholder",
      "email": "user@example.com",
    }
  })))
}