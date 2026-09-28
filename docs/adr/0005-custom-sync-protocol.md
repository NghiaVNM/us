# 0005 — Custom Sync Protocol

- Status: Accepted
- Decision: Per-space change log with server `seq`, push/pull with optimistic concurrency, client-side per-field LWW (HLC) merge, WebSocket pokes.
- Rejected: PowerSync, ElectricSQL (no value on encrypted payloads); CRDT libraries (overkill).
- Consequences: Full ownership of edge cases; property-based tests required.
