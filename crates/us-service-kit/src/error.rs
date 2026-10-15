//! Common error types for services.

use thiserror::Error;

/// Result type for service operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Service error types.
#[derive(Debug, Error)]
pub enum Error {
  /// Configuration error.
  #[error("Configuration error: {0}")]
  Config(String),

  /// HTTP client error.
  #[error("HTTP error: {0}")]
  Http(String),

  /// Authentication error.
  #[error("Authentication error: {0}")]
  Auth(String),

  /// Internal error.
  #[error("Internal error: {0}")]
  Internal(String),
}

impl Error {
  /// Create a config error.
  pub fn config(msg: impl Into<String>) -> Self {
    Self::Config(msg.into())
  }

  /// Create a HTTP error.
  pub fn http(msg: impl Into<String>) -> Self {
    Self::Http(msg.into())
  }

  /// Create an auth error.
  pub fn auth(msg: impl Into<String>) -> Self {
    Self::Auth(msg.into())
  }

  /// Create an internal error.
  pub fn internal(msg: impl Into<String>) -> Self{
    Self::Internal(msg.into())
  }
}