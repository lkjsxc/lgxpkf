# Note URL Scheme

## Format

- /{base32_id} (HTML note view)
- /notes/{base32_id} (JSON note payload)
- /notes/{base32_id}/related (JSON note + related)

## Examples

- /abcd1234...
- /notes/abcd1234...
- /notes/abcd1234.../related

## Validation

- Reject non-base32 characters.
- Enforce expected length for 32-byte IDs.
