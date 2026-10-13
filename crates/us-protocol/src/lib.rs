//! Protocol buffer types of Us.

pub mod common {
  pub mod v1 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../gen/rust/us.common.v1.rs"));
  }
}

pub mod keys {
  pub mod v1 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../gen/rust/us.keys.v1.rs"));
  }
}

// Re-export commonly used types.
pub use common::v1::{Error, Uuid};
pub use keys::v1::{KeyBundle, PublicKeys};