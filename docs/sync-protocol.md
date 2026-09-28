# Us — Sync Protocol

## 1. Principles

- Local SQLite is the UI source of truth.
- Server stores ciphertext, assigns order, enforces authorization.
- Merge happens on the client.
- Space = connection.

## 2. Record

```
Record {
  id        UUIDv7, client-generated
  space_id  connection ID
  type      event | counter | post | comment | reaction | ...
  version   server-assigned, per record
  seq       server-assigned, monotonic per space (cursor)
  meta      plaintext fields per E2E classification
  payload   ciphertext: fields + per-field HLC timestamps
  deleted   tombstone flag
}
```

## 3. Client Components

| Component | Role |
|---|---|
| Local DB | SQLite (native) / sqlite-wasm OPFS (web) |
| Outbox | Pending mutations, ordered |
| Media queue | Pending uploads, resumable |
| Cursor store | Last `seq` per space |
| HLC | Hybrid logical clock per device |

## 4. Operations

### 4.1 Write

1. Apply to local DB.
2. Enqueue mutation with unique `mutation_id`.

### 4.2 Push

- Request: `mutation_id`, record, `base_version`.
- Server: idempotent on `mutation_id`.
- Match: accept, assign `version` + `seq`, emit `record.changed`.
- Mismatch: reject with current record.

### 4.3 Conflict

1. Decrypt local and server versions.
2. Merge per-field LWW by HLC.
3. Re-push with new `base_version`.

### 4.4 Pull

- Request: `[(space_id, since_seq)]`, page size.
- Response: records ordered by `seq`, `has_more`.

### 4.5 Realtime

- WebSocket poke: `(space_id, latest_seq)`.
- Client pulls on poke.
- Background: silent push (FCM / Web Push) triggers pull.

### 4.6 Bootstrap

- Trigger: new device, or cursor older than compaction horizon.
- Snapshot of latest records per space, then resume log.

### 4.7 Compaction

- Keep latest version per record.
- Tombstones retained N days (admin config), then purged.

## 5. Out-of-band Data

| Data | Path |
|---|---|
| Media | Presigned multipart upload/download to Garage; record holds blob reference |
| Location | `location` service + `realtime`; latest only in NATS KV with TTL; not in change log |

## 6. Authorization

- Push/pull only for spaces where user is an active member.
- Archived: pull allowed up to archive `seq`; push denied except author deletes.
- Feature opt-in enforced per record type.

## 7. Versioning

- Protocol messages in `proto/`, `/v1`.
- `buf breaking` in CI.
- Payload schema versioned inside ciphertext.
- Min client version from admin config.
