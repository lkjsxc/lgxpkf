# Versions API

## Endpoint

- POST /notes/{base32_id}/versions
  - auth: required (Bearer token).
  - body: { value: string }
  - behavior:
    - Only the author of the source note can create a version.
    - Rejects if the source note already has a newer version.
    - Creates the new note chain and the version association atomically.
  - returns: { root: note, segments: [base32_id] }

## Errors

- 403 edit_forbidden: note is not owned by the caller.
- 409 version_exists: newer version already exists.
- 422 account_note_locked: account bootstrap notes cannot be versioned.
