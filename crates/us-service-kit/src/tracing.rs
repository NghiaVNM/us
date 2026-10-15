//! Tracing setup utilities.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing with environment filter.
///
/// Uses `RUST_LOG` environment variable for filtering.
/// Defaults to `info` level if not set.
pub fn init_tracing() {
  tracing_subscriber::registry()
    .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
    .with(tracing_subscriber::fmt::layer())
    .init();
}