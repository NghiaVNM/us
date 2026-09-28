# Design — Sync Details

Status: Accepted

## 1. Storage Model (server)

- `records` holds latest version only; update in place with new `seq`.
- Pull = `WHERE space_id = ? AND seq > ? ORDER BY seq`.
- Compaction implicit; only tombstones need purging.
- `tombstone_horizon_seq` per space; cursor below horizon → bootstrap.
- Seq assignment: `UPDATE spaces SET last_seq = last_seq + 1 RETURNING last_seq` (per-space serialization).

## 2. HLC

| Part | Bits |
|---|---|
| Physical (ms) | 48 |
| Logical | 16 |
| Tie-break | device_id (UUID) |

Max tolerated drift: 60 s ahead of local clock; beyond → clamp.

## 3. Payload Fields

```
field = { value, hlc, device_id }
```

Merge: per-field max `(hlc, device_id)`.

## 4. Deletes

- Delete wins over concurrent edits.
- Tombstone keeps `id`, `type`, `deleted`, HLC; payload cleared.

## 5. Push

| Case | Server result |
|---|---|
| New (`base_version = 0`, id unused) | `accepted(version=1, seq)` |
| `base_version == version` | `accepted(version+1, seq)` |
| `base_version != version` | `conflict(current record)` |
| Duplicate `mutation_id` | Original result |
| Not member / feature off / archived | `forbidden` |

- Batch: max 100 mutations; results per mutation.
- Idempotency records retained 30 days.

## 6. Pull

- Request: list of `(space_id, since_seq)`; limit 500 records.
- Response: records, `next_seq` per space, `has_more`, `bootstrap_required` per space.

## 7. Client Loop

1. On start / poke / reconnect / outbox non-empty: push outbox.
2. Resolve conflicts, re-push.
3. Pull until `has_more = false`.
4. Backoff: exponential, max 60 s, jitter.
