# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - kind is a single token (no whitespace).
  - common kinds: next, prev, version, link, reply, parent, child, author.
  - version links from account bootstrap notes are rejected.
  - version links are rejected if the source note already has a newer version (409 version_exists).
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
