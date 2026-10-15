//! Service configuration.

use serde::Deserialize;

/// Service configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  /// Server configuration.
  pub server: ServerConfig,

  /// Kratos configuration.
  pub kratos: KratosConfig,
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
  /// HTTP port.
  #[serde(default = "default_port")]
  pub port: u16,
}

/// Kratos configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct KratosConfig {
  /// Kratos public URL.
  pub public_url: String,

  /// Kratos admin URL.
  pub admin_url: String,
}

fn default_port() -> u16 {
  3000
}

impl Config {
  /// Load configuration from environment.
  pub fn load() -> anyhow::Result<Self> {
    let env = std::env::var("ENV").unwrap_or_else(|_| "dev".to_string());

    let config = us_service_kit::config::load_config(
      "config",
      &env,
      "AUTH",
    )?;

    Ok(config)
  }
}