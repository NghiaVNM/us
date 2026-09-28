# 0015 — Monorepo

- Status: Accepted
- Decision: Single GitHub repository; one Cargo workspace; `proto/`, `crates/`, `services/`, `apps/`, `deploy/`, `docs/`.
- Consequences: Atomic contract changes; CI builds affected targets only.
