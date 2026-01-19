# Compliance Requirements

## Data Handling

- Persist user and note data in PostgreSQL.
- Follow data minimization: store only required fields.

## Auth Provider

- Use Google OAuth2 OpenID Connect.
- Validate issuer, audience, and token expiry.

## Auditability

- Record key events: login, note creation, association creation.
- Ensure logs omit secrets and tokens.
