# 0012 — API Protocols

- Status: Accepted
- Decision: Client API: HTTP/2 + protobuf (`/v1`); realtime: WebSocket + protobuf; admin: REST/JSON + OpenAPI; internal: gRPC; async: NATS JetStream.
- Rejected: JSON client API (base64 overhead on ciphertext).
- Consequences: Shared `proto/` contracts; `buf breaking` in CI.
