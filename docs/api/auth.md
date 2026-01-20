# Auth API

## Endpoints

- POST /auth/google
  - Accepts Google ID token.
  - Requires policy_acceptance with accepted=true and version.
  - Returns session token.

- GET /auth/me
  - Returns current user profile.
  - Profile includes account_note_id when set.

- POST /auth/google/redirect
  - GIS redirect mode endpoint.
  - Requires policy acceptance in the state payload.
