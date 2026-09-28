# Us — End-to-End Encryption

## 1. Threat Model

| Adversary | Protected |
|---|---|
| Database / VPS breach | Content, keys |
| Hosting provider | Content, keys |
| Operator (admin) | Content, keys |
| Malicious server adding members | Signed membership log |
| Network attacker | TLS + E2E |

Not protected: metadata (§5), compromised client device, local data at rest beyond OS protection (web OPFS unprotected), malicious web JS served by compromised server.

## 2. Primitives

Library: audited pure-Rust RustCrypto crates (see `design/crypto.md`). No custom primitives.

| Purpose | Algorithm |
|---|---|
| Data encryption | XChaCha20-Poly1305 |
| Media streams | STREAM (XChaCha20-Poly1305), chunked |
| Key wrapping (asymmetric) | X25519 sealed box |
| Signatures | Ed25519 |
| Password KDF | Argon2id |

Every ciphertext carries a crypto version header.

## 3. Key Hierarchy

```
Encryption passphrase ──Argon2id──► KEK ─┐
Recovery key (24 words) ─────────────────┼─► masterKey (server stores wrapped copies only)
                                              │
                                              ├─► identity keypair (X25519 + Ed25519)
                                              │     private part encrypted by masterKey
                                              │
Connection ──► epochKey[n] (rotated on membership change)
                 sealed per member identity key
                    │
                    └─► itemKey (per record / media / location update)
                           └─► payload
```

## 4. Flows

### 4.1 Account

- Encryption passphrase is separate from login credentials.
- Setup: generate masterKey, identity keypair, recovery key; upload wrapped copies to `keys`.
- Recovery key resets passphrase.
- Losing both passphrase and recovery key = data loss (warned at setup).

### 4.2 Devices

| Flow | Method |
|---|---|
| New device | Passphrase, or QR from existing device (masterKey sealed to new device ephemeral key, relayed by server) |
| Web | Keys in IndexedDB wrapped by non-extractable WebCrypto key; new browser requires passphrase or QR |
| Lost device | Revoke sessions; optional masterKey rotation |

### 4.3 Connection Keys

| Event | Action |
|---|---|
| Create | epochKey[0], sealed to members |
| Add member | New epoch; newcomer gets current epoch only; selected history shared by re-wrapping itemKeys |
| Remove / leave | New epoch immediately; leaver gets no new data |
| Archive | Read-only locally |
| Delete | Server ciphertext removed; clients delete local copy (best effort) |
| Reconnect | New epoch; members keep old keys |

### 4.4 Membership Log

- Append-only hash chain per connection.
- Entry: `seq`, `prev_hash`, `action`, `actor`, `target`, `role`, `epoch`, `signature` (Ed25519 of actor).
- Actions: `create`, `add`, `remove`, `leave`, `role_change`, `transfer_owner`, `archive`, `dissolve`, `reconnect`.
- Clients verify chain and actor authority against role rules. Server verifies too; clients never trust server.

### 4.5 Identity Verification

- Default: TOFU.
- Invite link/QR carries inviter key fingerprint in URL fragment.
- Optional safety number / QR comparison.

## 5. Data Classification

| Data | Plaintext (server) | Encrypted |
|---|---|---|
| Common | User IDs, membership, timestamps, sizes, feature flags | — |
| Calendar | Start/end (UTC), all-day, timezone, RRULE, reminder offsets, participant IDs | Title, description, place |
| Day counter | Date, calendar type, milestones | Name, note |
| Diary | Media count, sizes | Text, media, media metadata (EXIF, GPS), comments, reactions, tags |
| Location | Update timestamp | Coordinates, accuracy, speed, heading, battery, charging |
| Profile | Display name, avatar reference, locale | — |

## 6. Notifications

- Clients attach an encrypted notification payload (epoch key) to notifying records.
- Server forwards payload opaquely.
- Android / web: decrypt and display on device.
- iOS: Notification Service Extension (v2); fallback localized generic text.

## 7. Media

- Client-side compression, thumbnails, encryption.
- Chunked STREAM for streaming playback.
- Server enforces size and quota; duration enforced client-side.

## 8. Limitations

- No forward secrecy (group-key model); upgrade path to MLS via version header.
- Removed members retain previously synced data.
- No server-side content moderation or search.
- Web E2E depends on integrity of served JS.
