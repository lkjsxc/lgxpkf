# Data Flow

## Note Creation

- Extract JSON payload with Actix.
- Validate size and schema.
- Create immutable note struct.
- Insert note and association records.
- Return base32 URL.

## Note Retrieval

- Extract base32 ID from path.
- Load note by ID.
- Return serialized note.
