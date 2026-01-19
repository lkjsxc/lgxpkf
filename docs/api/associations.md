# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - kind is a single token (no whitespace).
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
