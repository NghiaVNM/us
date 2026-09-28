# 0010 — Microservices, Event-driven

- Status: Accepted
- Decision: Services: `account`, `keys`, `connection`, `sync`, `media`, `location`, `realtime`, `scheduler`, `notification`, `admin`. Database per service. Transactional outbox.
- Consequences: Higher operational overhead; independent scaling on Kubernetes.
