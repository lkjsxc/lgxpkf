# Posting Notes from the Website

## Flow

- Sign in on the landing page.
- Exchange the Google ID token for a session token.
- POST /notes with the session token in the Authorization header.

## Payload

- {"value": "note text"}
- Value must be 1024 bytes or less.

## Result

- Response includes the base32 note ID.
- The note is accessible at /notes/{base32_id}.
