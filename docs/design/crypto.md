# Design — Crypto

Status: Accepted

## 1. Library

| Primitive | Crate (proposed, pure Rust, Wasm-compatible) |
|---|---|
| XChaCha20-Poly1305 | `chacha20poly1305` |
| Streaming AEAD | `aead::stream` (STREAM, XChaCha20-Poly1305) |
| X25519 / sealed box | `crypto_box` (`seal`, libsodium-compatible) |
| Ed25519 | `ed25519-dalek` |
| Argon2id | `argon2` |
| HKDF-SHA256 | `hkdf`, `sha2` |
| Hash | SHA-256 |
| Randomness | `getrandom` (OS / WebCrypto) |
| Mnemonic | BIP39 English wordlist |

All crypto lives in `us-crypto`. Secrets use `zeroize`.

## 2. Envelope

```
Envelope v1 (binary)
  version   u8     0x01
  alg       u8     0x01 = XChaCha20-Poly1305
  nonce     [24]   random
  body      [..]   ciphertext || tag(16)
```

AAD (never transmitted, reconstructed by reader):

```
"us/v1/" || purpose || 0x00 || context fields (length-prefixed)
```

| Purpose | Context |
|---|---|
| `master` | user_id, wrap_kind (`pass` / `recovery`) |
| `identity` | user_id, identity_version |
| `epoch` | connection_id, epoch |
| `item-key` | space_id, record_id, epoch |
| `record` | space_id, record_id, type |
| `media-key` | space_id, record_id, blob_id |
| `location` | connection_id, user_id, epoch, device_ts |
| `notify` | space_id, record_id, epoch |
| `link` | link_session_id |

## 3. Keys

| Key | Size | Source | Stored (server) |
|---|---|---|---|
| masterKey | 32 B | random | wrapped ×2 (`pass`, `recovery`) |
| passKEK | 32 B | Argon2id(passphrase, salt) | never |
| recoveryKEK | 32 B | HKDF(recovery entropy, "us/v1/recovery") | never |
| identity X25519 | 32 B | random | public; private wrapped by masterKey |
| identity Ed25519 | 32 B | random | public; private wrapped by masterKey |
| epochKey | 32 B | random | sealed per member |
| itemKey | 32 B | random | wrapped by epochKey |
| mediaKey | 32 B | random | wrapped by itemKey |

X25519 and Ed25519 are independent keypairs (no conversion).

## 4. KDF Parameters

| Param | Value |
|---|---|
| Algorithm | Argon2id v1.3 |
| Memory | 64 MiB |
| Iterations | 3 |
| Parallelism | 1 |
| Salt | 16 B random |

Params stored with salt; upgradable on next unlock.

## 5. Recovery Key

- 256-bit entropy → 24-word BIP39 mnemonic.
- Shown once at setup; confirmation required.

## 6. Fingerprint

- `fp = SHA-256("us/v1/fp" || ed25519_pub || x25519_pub)`.
- Invite fragment: base32(fp[0..20]).
- Safety number (two users): 60 digits from sorted fingerprints.

## 7. Device Linking (QR)

1. New device: ephemeral X25519 keypair; `keys.CreateLinkSession` → `session_id` (TTL 5 min).
2. QR: `session_id`, ephemeral public key.
3. Old device: seal masterKey to ephemeral key (AAD `link`); `keys.CompleteLinkSession`.
4. New device: fetch, open, discard ephemeral key.

## 8. Web Key Storage

- Non-extractable AES-GCM `CryptoKey` in IndexedDB.
- masterKey wrapped with it; unwrapped into Wasm memory while unlocked.
- Lock / logout: zeroize Wasm memory, delete wrapped copy on logout.

## 9. Records

- Per record: new itemKey.
- `payload = Envelope(itemKey, record body, AAD record)`.
- `wrapped_item_keys = [{ epoch, Envelope(epochKey[epoch], itemKey, AAD item-key) }]`.
- Sharing history with newcomers: append wrap for current epoch.

## 10. Media

- Per blob: mediaKey, wrapped by itemKey (AAD `media-key`).
- STREAM, plaintext chunk 64 KiB; random-access decryption by chunk index.
- Thumbnail: separate blob, own mediaKey.

## 11. Location & Notifications

- Location: `Envelope(epochKey[current], payload, AAD location)` per connection.
- Notification payload: `Envelope(epochKey[current], payload, AAD notify)`.

## 12. Local Data at Rest

- Decrypted data in local SQLite (search, performance).
- Protection: iOS Data Protection, Android FBE. Web OPFS: none.
- Keys in local DB remain wrapped by masterKey.
