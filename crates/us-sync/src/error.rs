//! Error type for sync service.

use thiserror::Error;

/// Result type for sync operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Sync service errors.
#[derive(Debug, Error)]
pub enum Error {
  /// Database error.
  #[error("Database error: {0}")]
  Database(#[from] sqlx::Error),

  /// Serialization error.
  #[error("Serialization error: {0}")]
  Serialization(#[from] serde_json::Error),

  /// Invalid event.
  #[error("Invalid event: {0}")]
  InvalidEvent(String),

  /// Conflict resolution failed.
  #[error("Conflict resolution failed: {0}")]
  ConflictResolution(String),
}