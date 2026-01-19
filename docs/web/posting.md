# Posting Notes from the Website

## Flow

- Sign in from the top bar Login button.
- Exchange the Google ID token for a session token.
- Tap Post (bottom-left) to open the full-screen composer.
- POST /notes with the session token in the Authorization header.
- On note pages, Edit opens the same composer prefilled with the current note content.

## Payload

- {"value": "note text"}
- Value can exceed 1024 bytes; it will be split into chained notes.
- Markdown is supported and rendered on the note page.

## Result

- Response includes the base32 root note ID and segment IDs.
- The post is accessible at /{base32_id} (the root note).
