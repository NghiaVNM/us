//! Auth service error types.

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

/// Auth service result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Auth service errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Unauthorized.
  #[error("Unauthorized: {0}")]
  Unauthorized(String),

  /// Internal error.
  #[error("Internal error: {0}")]
  Internal(String),

  /// Kratos error.
  #[error("Kratos error: {0}")]
  Kratos(String),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let (status, message) = match self {
      Error::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
      Error::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
      Error::Kratos(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Kratos: {}", msg)),
    };

    let body = Json(json!({
      "error": message,
    }));

    (status, body).into_response()
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::Kratos(err.to_string())
  }
}