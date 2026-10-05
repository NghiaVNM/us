//! Envelope encryption wrapper.

use chacha20poly1305::{
  aead::{Aead, KeyInit, Payload},
  XChaCha20Poly1305, XNonce,
};
use zeroize::Zeroizing;

use crate::{ALG_XCHACHA20_POLY1305, VERSION};

/// Encrypted envelope.
#[derive(Clone)]
pub struct Envelope {
  /// Protocol version.
  pub version: u8,
  /// Algorithm identifier.
  pub alg: u8,
  /// Nonce.
  pub nonce: [u8; 24],
  /// Ciphertext + tag.
  pub body: Vec<u8>,
}

impl Envelope {
  /// Encrypt plaintext with key and AAD.
  pub fn seal(key: &[u8; 32], plaintext: &[u8], aad: &[u8]) -> Result<Self, Error> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = Self::generate_nonce();

    let payload = Payload { msg: plaintext, aad };
    let body = cipher
      .encrypt(&nonce.into(), payload)
      .map_err(|_| Error::EncryptionFailed)?;

    Ok(Self {
      version: VERSION,
      alg: ALG_XCHACHA20_POLY1305,
      nonce,
      body,
    })
  }

  /// Decrypt ciphertext with key and AAD.
  pub fn open(&self, key: &[u8; 32], aad: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
    if self.version != VERSION {
      return Err(Error::UnsupportedVersion(self.version));
    }
    if self.alg != ALG_XCHACHA20_POLY1305 {
      return Err(Error::UnsupportedAlgorithm(self.alg));
    }

    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce: XNonce = self.nonce.into();

    let payload = Payload {
      msg: &self.body,
      aad,
    };

    let plaintext = cipher
      .decrypt(&nonce, payload)
      .map_err(|_| Error::DecryptionFailed)?;

    Ok(Zeroizing::new(plaintext))
  }

  /// Serialize to binary.
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(2 + 24 + self.body.len());
    buf.push(self.version);
    buf.push(self.alg);
    buf.extend_from_slice(&self.nonce);
    buf.extend_from_slice(&self.body);
    buf
  }

  /// Deserialize from binary.
  pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
    if data.len() < 26 {
      return Err(Error::InvalidEnvelope);
    }

    let version = data[0];
    let alg = data[1];
    let nonce: [u8; 24] = data[2..26].try_into().unwrap();
    let body = data[26..].to_vec();

    Ok(Self {
      version,
      alg,
      nonce,
      body,
    })
  }

  fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    getrandom::getrandom(&mut nonce).expect("RNG failure");
    nonce
  }
}

/// Envelope errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Unsupported protocol version.
  #[error("unsupported protocol version: {0}")]
  UnsupportedVersion(u8),

  /// Unsupported algorithm.
  #[error("unsupported algorithm: {0}")]
  UnsupportedAlgorithm(u8),

  /// Encryption failed.
  #[error("encryption failed")]
  EncryptionFailed,

  /// Decryption failed (authentication or corruption).
  #[error("decryption failed")]
  DecryptionFailed,

  /// Invalid envelope format.
  #[error("invalid envelope format")]
  InvalidEnvelope,
}