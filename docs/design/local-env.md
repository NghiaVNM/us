# Design — Local Environment (Compose)

Status: Accepted

## 1. Networks

| Network | Members |
|---|---|
| `edge` | traefik, web, auth-ui, kratos (public), hydra (public), account, keys, connection, sync, realtime |
| `internal` | all services, postgres, nats, garage, kratos (admin), hydra (admin), mailpit |

Admin ports of Kratos/Hydra, Postgres, NATS, Garage: `internal` only.

## 2. Containers

| Container | Image | Notes |
|---|---|---|
| traefik | traefik (pinned) | Local TLS via mkcert certs |
| postgres | postgres 18 | Init: DB + role per service, kratos, hydra |
| nats | nats (JetStream on) | KV buckets created at startup |
| garage | garage | Single node, dev layout |
| kratos | oryd/kratos | Identity schema: email, name; methods: password, passkey, oidc (Google), totp, lookup_secret |
| hydra | oryd/hydra | JWT access tokens; login/consent → `account` |
| mailpit | axllent/mailpit | Dev SMTP |
| account, keys, connection, sync, realtime | local builds | Migrations on start |

## 3. Hydra Clients

| Client | Type | Flow |
|---|---|---|
| `us-web` | public | Authorization Code + PKCE |
| `us-cli` | public | Authorization Code + PKCE, loopback redirect |

## 4. Hosts (dev)

`app.us.localhost`, `auth.us.localhost`, `api.us.localhost`, `media.us.localhost`.

Passkey RP ID on `.localhost`: verify in M0.4.

## 5. Volumes

`pg-data`, `nats-data`, `garage-meta`, `garage-data`, `certs`.
