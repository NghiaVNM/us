# 0006 — Rust Shared Core + Native UIs

- Status: Accepted
- Decision: `us-core` in Rust (crypto, sync, storage API, API client) via UniFFI (iOS, Android) and wasm-bindgen (web). UIs: SwiftUI, Compose, Svelte.
- Rejected: Native-only logic (3× implementation); KMP; Flutter / React Native / Compose Multiplatform.
- Consequences: Cross-compilation and FFI complexity. Fits memory-limited iOS extensions.
