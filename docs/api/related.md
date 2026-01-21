# Related Notes API

## GET /notes/{base32_id}/related

- Returns related notes linked by associations.
- Response contains associations with linked note payloads.
- Cross-author associations are limited to link, reply, and quote.
- Used by the note page to surface versions and linked notes.
