# Posting Notes from the Website

## Flow

- Sign in from the top bar Login button.
- Exchange the Google ID token for a session token.
- Tap Post (bottom-left) to open the full-screen composer.
- POST /notes with the session token in the Authorization header.

## Payload

- {"value": "note text"}
- Value must be 1024 bytes or less.
- Markdown is supported and rendered on the note page.

## Result

- Response includes the base32 note ID.
- The note is accessible at /{base32_id}.
