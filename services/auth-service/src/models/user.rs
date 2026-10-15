//! User identity model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User identity from Kratos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  /// User ID.
  pub id: Uuid,

  /// Email address.
  pub mail: String,

  /// User traits (additional metadata).
  #[serde(default)]
  pub traits: serde_json::Value,
}