# Design — Connection & Membership

Status: Accepted

## 1. Membership Log Entry

| Field | Type |
|---|---|
| `connection_id` | UUIDv7 |
| `seq` | u64, starts at 0 |
| `prev_hash` | [32], zero for `seq` 0 |
| `actor_id` | user ID |
| `action` | enum |
| `body` | action-specific |
| `signature` | Ed25519 by actor |

- Signed bytes: `"us/v1/log" || canonical(connection_id, seq, prev_hash, actor_id, action, body)`.
- Canonical encoding: fixed field order, length-prefixed (not protobuf).
- `entry_hash = SHA-256(signed bytes || signature)`.
- Append requires `prev_hash == head`; else rejected, client rebases and re-signs.

## 2. Actions

| Action | Body | Authorized actor |
|---|---|---|
| `create` | type, creator keys fingerprint | creator |
| `invite` | invite_id, token_hash, role, max_uses, expires_at | couple: member; group: owner/admin (member if allowed) |
| `revoke_invite` | invite_id | invite creator, owner/admin |
| `add` | user_id, keys fingerprint, role, invite_id | couple: member; group: owner/admin (member if allowed) |
| `remove` | user_id | owner/admin (not owner; admin cannot remove admin) |
| `leave` | — | self (owner must transfer first) |
| `role_change` | user_id, role | owner |
| `transfer_owner` | user_id | owner |
| `set_policy` | members_can_invite | owner |
| `rotate_epoch` | epoch, recipients hash | any active member |
| `archive` | — | couple: either |
| `dissolve` | — | group: owner |
| `reconnect` | — | couple: both (two entries) |

## 3. Epoch Rules

- Epoch recipients = active members.
- After member removal/leave: writers must `rotate_epoch` before writing.
- Rotation by remaining member only.
- `add` followed by `rotate_epoch` by the same actor.

## 4. Join Flow

1. Inviter appends `invite`; shares `https://app.<domain>/i/<token>#<fp>`.
2. Invitee resolves token → connection preview + inviter public keys; verifies `fp`.
3. Invitee submits join request (own public keys).
4. Any authorized online member device: verify, append `add` + `rotate_epoch`, seal epoch key.
5. Invitee receives membership + epoch key.

Pending until an authorized member is online (mobile: triggered by push).

## 5. Features

- Stored in `connection` service; not in signed log.
- `feature.enabled(connection, feature)`, `opt_in(connection, user, feature)`.
- Enforced by server delivery (sync, location) and honored by clients.

Features: `calendar`, `counter`, `location`, `diary`.
