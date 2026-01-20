# Posting Notes from the Website

## Flow

- Sign in from the top bar Login button.
- Exchange the Google ID token for a session token.
- Tap Post (next to the brand) to open the full-screen composer (hidden while signed out).
- Shortcut: press `n` to open the composer when Post is visible.
- Shortcut: press Ctrl+Enter to submit the composer.
- POST /notes with the session token in the Authorization header.
- On note pages, Edit opens the same composer prefilled with the current note content (except account bootstrap notes).

## Payload

- {"value": "note text"}
- Value can exceed 1024 bytes; it will be split into chained notes.
- Markdown is supported and rendered on the note page.

## Result

- Response includes the base32 root note ID and segment IDs.
- The post is accessible at /{base32_id} (the root note).
- The root note is automatically associated with the poster account note (kind: author).
