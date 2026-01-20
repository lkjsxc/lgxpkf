# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - kind is a single token (no whitespace).
  - common kinds: next, prev, version, link, reply, parent, child, author.
  - version links from account bootstrap notes are rejected.
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
