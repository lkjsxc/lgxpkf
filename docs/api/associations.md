# Associations API

## Endpoints

- POST /associations
  - body: { kind, from_id, to_id }
  - returns: association

- GET /associations?note={id}
  - returns: associations involving the note
