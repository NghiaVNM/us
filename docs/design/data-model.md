# Design — Data Model (Phase 0)

Status: Accepted. All services include `outbox(id, subject, payload, created_at, published_at)`.

## 1. `account`

| Table | Columns |
|---|---|
| `profiles` | `user_id` PK (Kratos ID), `display_name`, `avatar_blob_id`, `locale`, `created_at`, `updated_at`, `deleted_at` |

## 2. `keys`

| Table | Columns |
|---|---|
| `key_bundles` | `user_id` PK, `kdf_salt`, `kdf_params`, `master_by_pass`, `master_by_recovery`, `identity_version`, `x25519_pub`, `ed25519_pub`, `x25519_priv_enc`, `ed25519_priv_enc`, `created_at`, `updated_at` |
| `link_sessions` | `id` PK, `user_id`, `eph_pub`, `sealed_master`, `expires_at`, `created_at` |
| `devices` | `id` PK, `user_id`, `name`, `platform`, `created_at`, `last_seen_at`, `revoked_at` |

## 3. `connection`

| Table | Columns |
|---|---|
| `connections` | `id` PK, `type`, `state`, `head_seq`, `head_hash`, `current_epoch`, `members_can_invite`, `created_at` |
| `members` | PK(`connection_id`, `user_id`), `role`, `state`, `joined_seq`, `left_seq`, `archived_at`, `deleted_at` |
| `log_entries` | PK(`connection_id`, `seq`), `prev_hash`, `entry_hash`, `actor_id`, `action`, `signed_body`, `signature`, `created_at` |
| `epoch_keys` | PK(`connection_id`, `epoch`, `recipient_id`), `sealed_key`, `created_by`, `created_at` |
| `invites` | `id` PK, `connection_id`, `token_hash` UNIQUE, `role`, `max_uses`, `uses`, `expires_at`, `revoked_at`, `created_by` |
| `join_requests` | `id` PK, `invite_id`, `connection_id`, `user_id`, `x25519_pub`, `ed25519_pub`, `state`, `created_at` |
| `features` | PK(`connection_id`, `feature`), `enabled`, `updated_by`, `updated_at` |
| `opt_ins` | PK(`connection_id`, `user_id`, `feature`), `opted_in`, `updated_at` |

## 4. `sync`

| Table | Columns |
|---|---|
| `spaces` | `id` PK, `last_seq`, `tombstone_horizon_seq` |
| `records` | PK(`space_id`, `id`), `type`, `version`, `seq`, `author_id`, `meta` (jsonb), `payload`, `wrapped_item_keys`, `deleted`, `created_at`, `updated_at`; INDEX(`space_id`, `seq`) |
| `mutations` | `mutation_id` PK, `space_id`, `record_id`, `result`, `created_at` |
| `space_members` | PK(`space_id`, `user_id`), `state`, `archived_at_seq`, `features` (read model from connection events) |

## 5. `realtime`

Stateless. NATS KV buckets:

| Bucket | Key | TTL |
|---|---|---|
| `presence` | `user_id.device_id` | 60 s (heartbeat) |
| `watchers` | `connection_id.user_id` | 30 s (heartbeat) |

## 6. Client (SQLite)

| Table | Purpose |
|---|---|
| `records` | Decrypted records + field HLCs |
| `records_fts` | FTS5 index |
| `outbox` | Pending mutations |
| `cursors` | `space_id`, `seq` |
| `connections`, `members`, `log_entries` | Verified membership state |
| `keys` | Epoch keys (wrapped by masterKey) |
| `media_cache` | Blob ID, path, size, last access |
