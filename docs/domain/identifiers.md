# Identifiers

## Note ID

- 32 bytes of entropy.
- Stored as binary in PostgreSQL.
- Exposed as base32 in URLs.

## Association ID

- Derived from (kind, from_id, to_id, created_at) or generated randomly.
