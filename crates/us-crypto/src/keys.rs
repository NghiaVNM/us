//! Key types and operations.

use ed25519_dalek::{SigningKey, VerifyingKey};
use zeroize::{Zeroize, Zeroizing};

/// Master key (32 bytes).
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct MasterKey([u8; 32]);

impl MasterKey {
  /// Generate a new random master key.
  pub fn generate() -> Self {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).expect("RNG failure");
    Self(key)
  }

  /// Get key bytes.
  pub fn as_bytes(&self) -> &[u8; 32] {
    &self.0
  }

  /// Create from bytes.
  pub fn from_bytes(bytes: [u8; 32]) -> Self {
    Self(bytes)
  }
}

/// Identity keypair (Ed25519 for signing, x25519 for encryption).
pub struct IdentityKeypair {
  /// Ed25519 signing key.
  pub signing_key: SigningKey,
  /// Ed25519 veirfying key.
  pub verifying_key: VerifyingKey,
}

impl IdentityKeypair {
  /// Generate a new random keypair.
  pub fn generate() -> Self {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();

    Self {
      signing_key,
      verifying_key,
    }
  }

  /// Get signing key bytes.
  pub fn signing_bytes(&self) -> Zeroizing<[u8; 32]> {
    Zeroizing::new(self.signing_key.to_bytes())
  }

  /// Get verifying key bytes.
  pub fn verifying_bytes(&self) -> [u8; 32] {
    self.verifying_key.to_bytes()
  }
}