# Data Flow

## Note Creation

- Parse request body.
- Validate size and schema.
- Create immutable note struct.
- Insert note and association records.
- Return base32 URL.

## Note Retrieval

- Parse base32 ID.
- Load note by ID.
- Return serialized note.
