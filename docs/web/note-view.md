# Note View

## Route

- GET /{base32_id}

## Behavior

- Server renders the note content and metadata.
- Markdown is rendered into HTML for the main body.
- prev/next associations are traversed recursively and concatenated into one document.
- Newer versions (version associations) are surfaced above the note.
- Context, chain, and association tools appear below the note in a stacked layout.
- The palette is a darker, bluish mode with oversized note typography.
