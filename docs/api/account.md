# Account API

## POST /account/note

- Requires Authorization bearer token.
- body: { value }
- value is stored as a new immutable note.
- Updates account_note_id on the user profile.
- returns: note
