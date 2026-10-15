//! Configuration loading utilities.

use config::{Config, ConfigError, Environment, File};
use serde::de::DeserializeOwned;
use std::path::Path;

/// Load configuration from file and environment.
///
/// Loads from `config/{env}.toml` and environment variables with prefix.
pub fn load_config<T: DeserializeOwned> (
  config_dir: impl AsRef<Path>,
  env: &str,
  env_prefix: &str,
) -> Result<T, ConfigError> {
  let config_path = config_dir.as_ref().join(format!("{}.toml", env));

  Config::builder()
    .add_source(File::from(config_path).required(false))
    .add_source(Environment::with_prefix(env_prefix).separator("__"))
    .build()?
    .try_deserialize()
}