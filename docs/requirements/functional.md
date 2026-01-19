# Functional Requirements

## Authentication

- Users can log in with Google accounts.
- Access tokens are validated on each authenticated request.

## Notes

- Users can create notes with immutable content.
- Notes are retrievable by base32 URL.
- Notes list is queryable by author and time range.

## Associations

- Notes can be linked by a kind and two note IDs.
- Associations can represent versioning, replies, or aggregation.

## APIs

- HTTP APIs for notes, associations, and user session info.
- Errors are returned with stable codes and messages.
