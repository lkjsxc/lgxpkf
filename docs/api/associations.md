# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - kind is a single token (no whitespace).
  - allowed kinds: link, reply, quote, parent, child.
  - from_id must be authored by the caller.
  - to_id must exist; accepts a note ID or a note URL.
  - system-only kinds (author, next, prev, version) are rejected.
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
