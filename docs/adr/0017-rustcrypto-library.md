# 0017 — RustCrypto Library

- Status: Accepted
- Decision: Pure-Rust RustCrypto crates (`chacha20poly1305`, `aead::stream`, `crypto_box`, `ed25519-dalek`, `argon2`, `hkdf`) instead of C libsodium. Media uses STREAM instead of secretstream.
- Rejected: libsodium C bindings (Wasm build complexity).
- Consequences: Same algorithms; single build for native and Wasm.
