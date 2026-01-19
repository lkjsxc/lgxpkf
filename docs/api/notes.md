# Notes API

## Endpoints

- POST /notes
  - body: { value: string }
  - returns: note with base32 id

- GET /notes/{base32_id}
  - returns: note

- GET /notes/{base32_id}/related
  - returns: related notes and associations

- GET /notes?author={id}&from={ts}&to={ts}
  - returns: list of notes
