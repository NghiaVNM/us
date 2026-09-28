# 0018 — Local Plaintext Storage

- Status: Accepted
- Decision: Decrypted data stored in local SQLite; rely on OS at-rest protection. Keys stay wrapped.
- Rejected: SQLCipher / app-level local encryption (search and performance cost).
- Consequences: Web OPFS data readable with device access.
