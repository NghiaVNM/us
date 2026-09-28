# 0011 — Infrastructure Components

- Status: Accepted
- Decision: PostgreSQL, NATS JetStream (+ KV), Garage (S3), Traefik.
- Rejected: MinIO CE (archived April 2026); Kafka (heavy); Redis/Valkey (covered by NATS KV).
- Consequences: Minimal component count; all run on Compose and Kubernetes.
