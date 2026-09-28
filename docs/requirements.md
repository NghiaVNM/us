# Us — Requirements

## 1. Product

Private sharing app for couples and groups (friends, family). Offline-first, end-to-end encrypted.

## 2. Goals & Constraints

| Item | Value |
|---|---|
| Priorities | Best practice > performance > offline reliability > privacy > cost > delivery speed |
| Languages | Vietnamese, English |
| Theme | Light, dark |
| Team | Solo developer; goal: upskill |
| Hosting | Self-hosted; Docker Compose on VPS; Kubernetes-ready |
| SaaS | None paid, except SMTP email provider |
| Docs / comments | English |
| Current limits | No Mac, no Apple Developer account |

## 3. Platforms

| Platform | Min version | Phase |
|---|---|---|
| Web (PWA) | Latest 2 versions of Chrome, Edge, Firefox, Safari; iOS Safari 17+ | v1 |
| iOS | iOS 17 | v2 |
| Android | API 29 | v2 |

Build order: core + CLI → backend → web → iOS → Android.

## 4. Domain Model

- User has many connections.
- Connection types:
  - `couple`: exactly 2 members.
  - `group`: max members configurable (default 50).
- Max connections per user configurable (default unlimited).
- All features and data are connection-scoped. No data outside connections.

### 4.1 Feature Activation

- Any member enables a feature for a connection.
- Each member opts in individually.
- Non-opted-in member neither shares nor sees that feature's data.
- UI lists members not opted in; members can send a nudge.
- Disable for connection: couple — either member; group — owner/admin.

### 4.2 Roles

| Action | Couple | Group |
|---|---|---|
| Invite | Member (when 1 member) | owner/admin; member if owner allows |
| Remove member | — | owner/admin (admin cannot remove owner/admin) |
| Change role / transfer ownership | — | owner |
| Leave | Any | Any; owner must transfer first |
| Dissolve | Disconnect by either | owner |

### 4.3 Content Permissions

| Content | Edit | Delete |
|---|---|---|
| Calendar event | Any member | Any member |
| Day counter | Any member | Any member |
| Post, comment | Author | Author; group owner/admin |
| Reaction | Author | Author |

Authors can always delete own content, including in archived connections.

### 4.4 Lifecycle

| State / Event | Behavior |
|---|---|
| Couple disconnect | Archived (read-only) for both |
| Leave group | Archived for leaver only |
| Delete (archived) | Removes member access and local copy |
| Server data purge | When last member deletes, or after N days archived (admin config; default never) |
| Reconnect | Requires server data. Couple: both agree. Group: re-invite as new member |
| New member | No history by default; inviter selects items to share |

### 4.5 Invitations

- Link and QR code. No public user search.
- Expiring, max uses, revocable.
- Inviter key fingerprint in URL fragment.

## 5. Features

### 5.1 Calendar

- One calendar per connection.
- Timed and all-day events; timezone support; optional recurrence.
- Participants selected from members; only participants get reminders.
- Views:
  - Connection: all members' events.
  - Personal: aggregated events the user participates in, labeled by connection.
- Copy event to another connection (independent copy).
- No external calendar sync.

### 5.2 Day Counter

- Multiple counters per connection.
- Solar calendar date.
- User-defined milestones; notification on milestone.
- Home screen widget (v2).
- Lunar calendar: later.

### 5.3 Location Sharing

| Mode | Trigger | Default rate |
|---|---|---|
| Background (mobile) | Opted in | Android: ~10 min or >200 m; iOS: significant change |
| Realtime | A member views the map | ~5 s or >10 m; stops 2 min after last viewer |
| Web | Tab/PWA open | ~30 s or significant change |

- Latest location only, with timestamp ("updated X min ago").
- TTL 7 days; afterwards "location unknown".
- Payload: lat, lng, accuracy, speed, heading, battery, charging, device timestamp.
- Web: prompt user to enable.
- Map: MapLibre + self-hosted Protomaps planet tiles.
- No reverse geocoding.
- Rates and TTL admin-configurable.

### 5.4 Diary (Feed)

- Feed per connection.
- Text, photos, videos.
- Reactions, comments, member tags.
- Offline upload queue.
- Infinite scroll; older content lazy-loaded.
- Media limits admin-configurable.

### 5.5 Notifications

- v1: Web Push (incl. iOS PWA).
- v2: FCM, APNs.
- Localized to recipient language.

## 6. Accounts & Auth

- Sign-in: passkey, email + password, Google. Apple: v2. No SMS.
- MFA: required for admins, optional for users.
- Separate encryption passphrase + recovery key.
- Add device: passphrase or QR from existing device.
- Loss of passphrase and recovery key = permanent loss of E2E data.

## 7. Offline

- All text data stored locally.
- Media: LRU cache; size user-configurable.
- Local full-text search.
- UI never blocks on network.

## 8. Admin Portal

### 8.1 Configuration

| Group | Parameters (default) |
|---|---|
| Media | Max image size (20 MB), max video size (200 MB), max video duration (60 s), max files/post (10), compression quality |
| Quota | Storage per user (5 GB) |
| Connection | Max connections/user (unlimited), max group members (50), invite TTL (7 d), invite max uses |
| Location | Background rate, realtime rate, TTL (7 d) |
| Data | Archived connection auto-purge (never), tombstone retention |
| System | Global feature flags, maintenance mode, min client version, rate limits |
| Notification | Push templates per language |

### 8.2 User Management

- Search by email/ID.
- View metadata: created, last active, devices, connections, storage.
- Lock/unlock, revoke sessions, delete account.
- No content access.

### 8.3 Statistics (aggregated metadata)

- Users: total, sign-ups, DAU/WAU/MAU.
- Connections: count by type, feature adoption.
- Activity: events, posts, media per day.
- Storage: total, growth.
- Push delivery success rate.

### 8.4 Access

- Roles: `admin`, `viewer`.
- MFA required.
- Immutable audit log.
- Separate subdomain.

## 9. Scope

| Phase | Items |
|---|---|
| Phase 0 | Monorepo, CI, protobuf, `us-crypto`, `us-protocol`, `us-core`, CLI, auth, `account`, `keys`, `connection`, `sync`, `realtime`, Compose |
| v1 | Web PWA with all features in §5 (web limits), auth, admin portal |
| v2 | iOS, Android, widgets, background location, iOS Notification Service Extension, Sign in with Apple, albums, location history, geofence, ghost mode, temporary sharing, saved places, linked events across connections |
| Later | Lunar calendar, user report/feedback, MLS evaluation |

## 10. Testing (minimum)

| Level | Scope |
|---|---|
| Required | `us-crypto` test vectors + roundtrip; property-based tests for merge/HLC; `buf breaking` |
| CI | cargo-deny, cargo-audit, gitleaks |
| Later | Integration, Playwright, k6, fuzzing |

## 11. Infrastructure Estimate

| Resource | Initial |
|---|---|
| RAM | 2–4 GB |
| vCPU | 2 |
| Disk | ≥ 200 GB (planet tiles ~100+ GB, media) |
