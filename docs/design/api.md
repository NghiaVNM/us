# Design — API Contracts (Phase 0)

Status: Accepted. Defined in `proto/us/<service>/v1/`.

## 1. Conventions

- Transport: HTTP/2 `POST /v1/<service>/<Method>`, body protobuf (`application/x-protobuf`).
- Errors: HTTP status + `us.common.v1.Error { code, message, details }`.
- Auth: `Authorization: Bearer <JWT>`.
- IDs: UUIDv7 as 16 bytes.
- Timestamps: `int64` ms UTC.

## 2. `account`

| Method | Purpose |
|---|---|
| `GetMe` | Profile |
| `UpdateProfile` | Display name, avatar, locale |
| `DeleteAccount` | Start account purge |
| Internal | Hydra login/consent handlers; Kratos webhooks |

## 3. `keys`

| Method | Purpose |
|---|---|
| `SetupKeyBundle` | First-time upload (wrapped master keys, identity keys) |
| `GetMyKeyBundle` | Fetch wrapped keys for unlock |
| `UpdatePassphraseWrap` | Change passphrase / reset via recovery |
| `GetPublicKeys` | Public keys + fingerprints for user IDs |
| `CreateLinkSession` / `CompleteLinkSession` / `GetLinkSession` | QR device linking |
| `ListDevices` / `RevokeDevice` | Device management |

## 4. `connection`

| Method | Purpose |
|---|---|
| `CreateConnection` | Type, `create` entry, epoch 0 keys |
| `ListConnections` | User's connections + state |
| `GetLog` | Entries since seq |
| `AppendLog` | Signed entry (+ epoch keys if `rotate_epoch`) |
| `ResolveInvite` | Token → preview + inviter keys |
| `RequestJoin` / `ListJoinRequests` | Join flow |
| `GetEpochKeys` | Sealed epoch keys for caller |
| `SetFeature` / `SetOptIn` / `GetFeatures` | Feature state |

## 5. `sync`

| Method | Purpose |
|---|---|
| `Push` | Mutations → per-mutation results |
| `Pull` | Cursors → records |
| `Snapshot` | Paged bootstrap per space |

## 6. `realtime` (WebSocket `/v1/ws`)

| Frame | Direction |
|---|---|
| `Hello { token }` / `Welcome` | C→S / S→C |
| `Poke { space_id, seq }` | S→C |
| `MembershipChanged { connection_id, seq }` | S→C |
| `WatchStart` / `WatchStop { connection_id }` | C→S |
| `WatchersChanged { connection_id, count }` | S→C |
| `LocationUpdate { connection_id, user_id, envelope, ts }` | S→C (v1) |
| `Ping` / `Pong` | both |

Token refresh: new `Hello` on same socket.

## 7. Events (NATS JetStream)

- Envelope: `id`, `type`, `source`, `time`, `subject`, `data` (protobuf).
- Subjects: `us.<domain>.<event>` (e.g. `us.sync.record_changed`).
- Delivery: at-least-once; consumers idempotent by `id`.

| Event | Producer |
|---|---|
| `connection.membership_changed` | connection |
| `connection.feature_changed` | connection |
| `sync.record_changed` | sync |
| `account.user_deleted` | account |
