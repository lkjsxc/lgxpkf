# Compliance Requirements

## Data Handling

- Persist user and note data in PostgreSQL.
- Follow data minimization: store only required fields.
- Avoid storing raw Google ID tokens.
- Document data retention windows and deletion workflows.

## Auth Provider

- Use Google OAuth2 OpenID Connect.
- Validate issuer, audience, and token expiry.

## Japan Privacy Alignment

- Maintain a privacy notice aligned with APPI and PPC guidelines.
- Disclose data categories, purposes, and storage region/cross-border transfers.
- Provide a user request workflow for access, correction, and deletion.
- Maintain incident response steps for PPC and user notifications.
- Publish Terms of Service and Guideline consistent with Japanese law.
- Require policy acceptance before account creation and sign-in completion.

## Policy Surface

- Serve /terms, /privacy, and /guideline on the web UI.
- Link to policy pages from signed-out and signed-in experiences.

## Auditability

- Record key events: login, note creation, association creation.
- Ensure logs omit secrets and tokens.
