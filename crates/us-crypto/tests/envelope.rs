//! Test for envelope encryption.

use us_crypto::Envelope;

#[test]
fn seal_then_open_roundtrip() {
  let key = [7u8; 32];
  let plaintext = b"hello us";
  let aad = b"us/v1/record";

  let envelope = Envelope::seal(& key, plaintext, aad).unwrap();
  let decrypted = envelope.open(&key, aad).unwrap();

  assert_eq!(decrypted.as_slice(), plaintext);
}

#[test]
fn open_with_wrong_key_fails() {
  let key = [7u8; 32];
  let wrong_key = [8u8; 32];
  let aad = b"us/v1/record";

  let envelope = Envelope::seal(&key, b"secret", aad).unwrap();
  let result = envelope.open(&wrong_key, aad);

  assert!(result.is_err());
}

#[test]
fn open_with_wrong_aad_fails() {
  let key = [7u8; 32];

  let envelope = Envelope::seal(&key, b"secret", b"us/v1/record").unwrap();
  let result = envelope.open(&key, b"us/v1/location");

  assert!(result.is_err());
}

#[test]
fn tampered_ciphertext_fails() {
  let key = [7u8; 32];
  let aad = b"us/v1/record";

  let mut envelope = Envelope::seal(&key, b"secret", aad).unwrap();
  envelope.body[0] ^= 0xff; // flip bits

  let result = envelope.open(&key, aad);

  assert!(result.is_err());
}

#[test]
fn serialize_deserialize_roundtrip() {
  let key = [7u8; 32];
  let aad = b"us/v1/record";

  let envelope = Envelope::seal(&key, b"payload", aad).unwrap();
  let bytes = envelope.to_bytes();
  let restored = Envelope::from_bytes(&bytes).unwrap();

  let decrypted = restored.open(&key, aad).unwrap();
  assert_eq!(decrypted.as_slice(), b"payload");
}

#[test]
fn from_bytes_rejects_short_input() {
  let result = Envelope::from_bytes(&[0u8; 10]);
  assert!(result.is_err());
}

#[test]
fn two_seals_produce_different_nonces() {
  let key = [7u8; 32];
  let aad = b"us/v1/record";

  let a = Envelope::seal(&key, b"same", aad).unwrap();
  let b = Envelope::seal(&key, b"same", aad).unwrap();

  // Different nonces -> different ciphertexts for identical plaintext
  assert_ne!(a.nonce, b.nonce);
  assert_ne!(a.body, b.body);
}