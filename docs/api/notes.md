# Notes API

## Endpoints

- POST /notes
  - body: { value: string }
  - value can exceed 1024 bytes; it will be split into 1024-byte segments.
  - returns: { root: note, segments: [base32_id] }

- GET /notes/{base32_id}
  - returns: note (JSON)

- GET /notes/{base32_id}/related
  - returns: related notes and associations

- GET /notes?author={id}&from={ts}&to={ts}
  - returns: list of notes

## Note View

- GET /{base32_id}
  - returns HTML note page with chained content, context, and associations
