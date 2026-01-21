# Notes API

## Endpoints

- POST /notes
  - body: { value: string }
  - value can exceed 1024 bytes; it will be split into 1024-byte segments.
  - returns: { root: note, segments: [base32_id] }
  - server links the root note to the poster account note (association kind: author).

- POST /notes/{base32_id}/versions
  - creates a new version of the specified note (see docs/api/versions.md).

- GET /notes/{base32_id}
  - returns: note (JSON)

- GET /notes/{base32_id}/related
  - returns: related notes and associations

- GET /notes?author={id}&from={ts}&to={ts}
  - returns: list of notes

- GET /notes/random?limit={n}
  - returns: list of random notes

## Note View

- GET /{base32_id}
  - returns HTML note page with chained content, context, and associations
