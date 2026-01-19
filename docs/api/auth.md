# Auth API

## Endpoints

- POST /auth/google
  - Accepts Google ID token.
  - Returns session token.

- GET /auth/me
  - Returns current user profile.
  - Profile includes account_note_id when set.
