# 0008 — Rust Backend

- Status: Accepted
- Decision: All custom services in Rust (axum, tokio, tower, tonic, sqlx, tracing).
- Rejected: Go (no code sharing with core).
- Consequences: Shared `us-protocol` crate with clients; small memory footprint; longer compile times.
