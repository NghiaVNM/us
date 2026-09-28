# 0002 — E2E Group-key Model with Epochs

- Status: Accepted
- Decision: Ente-style hierarchy (masterKey → epochKey per connection → itemKey). New epoch on every membership change. Signed membership log. RustCrypto primitives (ADR 0017).
- Rejected: MLS (history access conflict, complexity); server-side encryption only.
- Consequences: No forward secrecy. Versioned ciphertext allows later MLS migration.
