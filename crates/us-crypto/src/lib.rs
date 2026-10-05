//! Core cryptographic primitives for Us.
//!
//! This crate implements the E2E encryption scheme documented in `docs/security/e2e.md`
//! and `docs/design/crypto.md`

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod envelope;
pub mod keys;

pub use envelope::Envelope;
pub use keys::{IdentityKeypair, MasterKey};

/// Crypto protocol version.
pub const VERSION: u8 = 0x01;

/// XChaCha20-Poly1305 algorithm identifier.
pub const ALG_XCHACHA20_POLY1305: u8 = 0x01;