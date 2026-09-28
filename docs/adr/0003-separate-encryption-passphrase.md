# 0003 — Separate Encryption Passphrase

- Status: Accepted
- Decision: E2E keys protected by a dedicated passphrase (Argon2id) plus recovery key, independent of login.
- Rejected: Deriving keys from login password (IdP sees password; unavailable with social login).
- Consequences: Users manage an extra secret. QR device linking reduces friction.
