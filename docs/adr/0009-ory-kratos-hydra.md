# 0009 — Ory Kratos + Hydra

- Status: Accepted
- Decision: Kratos (identity, passkeys, social login, MFA) + Hydra (OAuth2/OIDC). Apps use Authorization Code + PKCE. Services validate JWT via JWKS.
- Rejected: Keycloak (heavy); Zitadel (heavier, AGPL); custom auth (security risk).
- Consequences: Custom login UI (`auth-ui`); `account` implements Hydra login/consent.
