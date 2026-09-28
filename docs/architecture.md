# Us — Architecture

## 1. Principles

- Bounded-context microservices; each service owns its data.
- Server never reads content (E2E); server logic uses plaintext metadata only.
- Event-driven with transactional outbox.
- 12-factor; stateless services; same images on Compose and Kubernetes.

## 2. Overview

```
 Clients (web · iOS · Android · admin-portal)
        │ HTTPS / WSS
 ┌──────▼──────┐
 │  Traefik    │  TLS, routing, rate limit
 └──┬───┬───┬──┘
    │   │   └──────────────► Kratos / Hydra ─┐
    │   │                                    │ JWKS
    │   ▼                                    ▼
    │ realtime (WS) ◄──NATS──► sync · connection · keys · account
    │                          media · location · admin
    │                               │ events (JetStream)
    │                               ▼
    │                     scheduler ──► notification ──► Web Push / FCM / APNs
    ▼
  Garage (S3, presigned direct upload/download, map tiles)

 PostgreSQL: one database + one role per service
```

## 3. Clients

```
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Android      │ │ iOS          │ │ Web          │
│ Kotlin +     │ │ Swift +      │ │ TypeScript + │
│ Compose      │ │ SwiftUI      │ │ Svelte 5     │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │ UniFFI         │ UniFFI         │ wasm-bindgen
┌──────▼────────────────▼────────────────▼───────┐
│ us-core (Rust)                                 │
│ crypto · sync engine · merge/HLC · storage API │
│ domain models · API client                     │
└────────────────────────────────────────────────┘
```

- `us-core` owns networking, sync, crypto. UI talks only to core.
- Storage is a trait; implemented per platform.

### 3.1 Web

```
Main thread           Dedicated/Shared Worker              Service Worker
┌───────────┐  msgs   ┌──────────────────────────────┐    ┌───────────────┐
│ Svelte UI │◄──────► │ us-core (Wasm)               │    │ PWA cache     │
└───────────┘         │ @sqlite.org/sqlite-wasm OPFS │    │ Web Push      │
                      └──────────────────────────────┘    └───────────────┘
```

| Item | Choice |
|---|---|
| Framework | Svelte 5 + SvelteKit (SPA, adapter-static) + Vite |
| i18n | Paraglide |
| SQLite VFS | `opfs-sahpool`; single owner tab via SharedWorker / Web Locks |
| Map | MapLibre GL JS |
| Lists | TanStack Virtual |
| UI kit | None (custom design) |

### 3.2 Mobile (v2)

| Item | iOS | Android |
|---|---|---|
| UI | SwiftUI (Observation) | Jetpack Compose |
| Core binding | UniFFI → xcframework | UniFFI → AAR |
| Widget | WidgetKit | Glance |
| Push | APNs + Notification Service Extension | FCM |
| Auth | ASWebAuthenticationSession + PKCE | Custom Tabs + PKCE |

## 4. Services

### 4.1 Custom (Rust)

| Service | Responsibility |
|---|---|
| `account` | Profile, locale; Hydra login/consent provider; Kratos webhooks |
| `keys` | Wrapped master keys, identity public keys, recovery, device registry |
| `connection` | Connections, signed membership log, roles, invites, feature flags/opt-ins, wrapped epoch keys |
| `sync` | Per-space change log, push/pull, conflict detection, compaction |
| `media` | Presigned URLs, multipart upload, quotas, orphan GC |
| `location` | Latest encrypted location (TTL), rate config |
| `realtime` | WebSocket gateway: sync pokes, location fan-out, presence/watchers |
| `scheduler` | Consumes record events; RRULE expansion; reminders, milestones |
| `notification` | Web Push (VAPID), FCM, APNs; device tokens; localized templates; retry |
| `admin` | Admin API: config (owner), user management via Kratos admin API, statistics read models, audit log |

### 4.2 Frontends

| App | Purpose |
|---|---|
| `web` | Main app (PWA) |
| `auth-ui` | Login, registration, recovery (Kratos flows) |
| `admin-portal` | Admin UI |

### 4.3 Third-party

| Component | Role |
|---|---|
| Ory Kratos | Identity, passkeys, social login, MFA, email flows |
| Ory Hydra | OAuth2 / OIDC provider |
| PostgreSQL | Primary database (CloudNativePG on K8s) |
| NATS JetStream | Event bus; KV for location, presence |
| Garage | S3-compatible object storage |
| Traefik | Edge proxy, ACME TLS, rate limiting |

## 5. Backend Stack

| Concern | Choice |
|---|---|
| Language | Rust |
| HTTP | axum, tokio, tower |
| gRPC | tonic, prost |
| DB | sqlx (compile-time checked), sqlx migrations |
| Telemetry | tracing, OpenTelemetry (OTLP) |
| Auth | Shared middleware: JWT validation via Hydra JWKS |

## 6. Communication

| Path | Protocol |
|---|---|
| Client ↔ backend | HTTP/2, protobuf bodies, `/v1` |
| Realtime | WebSocket, protobuf frames |
| Admin portal ↔ `admin` | REST/JSON, OpenAPI |
| Service ↔ service (sync) | gRPC; mTLS via mesh on K8s |
| Service ↔ service (async) | NATS JetStream + transactional outbox |
| AuthN | Every service validates JWT (zero trust) |

### 6.1 Key Events

| Event | Producer | Consumers |
|---|---|---|
| `record.changed` | sync | realtime, scheduler, admin |
| `membership.changed` | connection | sync, realtime, location, scheduler |
| `feature.changed` | connection | realtime, location, scheduler |
| `location.updated` | location | realtime |
| `notification.requested` | scheduler, connection, sync | notification |
| `config.changed` | admin | all |
| `user.deleted` | account | all |

## 7. Data

| Store | Usage |
|---|---|
| PostgreSQL | One instance on VPS; database + role per service; Kratos/Hydra own DBs |
| NATS KV | Latest location (TTL), watchers, presence |
| Garage | Encrypted media blobs; PMTiles planet file |

Backups: WAL-G or pgBackRest; Garage replication/off-site copy; encrypted; periodic restore drills.

## 8. Auth Flow

- Apps: OAuth2 Authorization Code + PKCE (RFC 8252) via Hydra; login UI via Kratos.
- Access token: short-lived JWT. Refresh token: rotating.
- Offline: app works locally; tokens refreshed when online.
- E2E keys never touch IdP.

## 9. Routing

| Host | Target |
|---|---|
| `app.<domain>` | web |
| `auth.<domain>` | auth-ui, Kratos public, Hydra public |
| `api.<domain>` | services (`/v1/...`), WebSocket |
| `media.<domain>` | Garage (presigned) |
| `tiles.<domain>` | Garage (PMTiles, range requests) |
| `admin.<domain>` | admin-portal, `admin` API |

## 10. Repository

```
us/
├── .github/workflows/     # CI
├── proto/us/<pkg>/v1/     # protobuf contracts: common, account, keys, connection, sync, realtime, events
├── crates/
│   ├── us-crypto/         # E2E primitives
│   ├── us-protocol/       # shared types, validation, membership log verification
│   ├── us-core/           # client core (sync, storage API, API client)
│   └── us-service-kit/    # backend shared: auth middleware, telemetry, outbox, config
├── services/              # account, keys, connection, sync, media, location, realtime, scheduler, notification, admin
├── apps/                  # web, auth-ui, admin-portal, ios, android, cli
├── deploy/
│   ├── compose/config/    # traefik, postgres, nats, garage, kratos, hydra
│   └── k8s/               # Helm/Kustomize (later)
├── scripts/
└── docs/
```

- Monorepo on GitHub; single Cargo workspace.

## 11. Delivery

| Concern | Choice |
|---|---|
| CI | GitHub Actions; lint, test, build affected; cargo-chef; sccache |
| Images | Distroless, non-root, read-only FS, no capabilities, pinned digests |
| Secrets | SOPS + age |
| GitOps (K8s) | ArgoCD |
| Health | `/healthz`, `/readyz`, `/metrics` |
| Logs | Structured JSON to stdout |

## 12. Operational Security

| Area | Measure |
|---|---|
| Network | Only Traefik exposed (80/443); internal network for all else; nftables; SSH keys only; unattended security updates |
| Rate limit | Traefik per IP; services per user (login, invites, upload) |
| Invites | 128-bit random tokens, hashed at rest, TTL, max uses, revocable, attempt limits |
| Web | Strict CSP (no inline), Trusted Types, SRI, HSTS |
| Logs | No content or PII; IDs only; truncated IPs; 14-day retention |
| Secrets | Hydra JWKS rotation; documented rotation for DB, S3, VAPID keys |
| Admin | MFA required; audit log |
| Account deletion | Full server-side purge with verification job |
