# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - kind is a single token (no whitespace).
  - allowed kinds: link, reply, quote, parent, child, next, prev.
  - from_id must be authored by the caller and cannot be the account note.
  - parent/child/next/prev require both notes to share the same author.
  - link/reply/quote may target notes from other authors.
  - to_id must exist; accepts a note ID or a note URL.
  - system-only kinds (author, version) are rejected.
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
