# Note View

## Route

- GET /{base32_id}

## Behavior

- Server renders the note content and metadata.
- Markdown is rendered into HTML for the main body.
- prev/next associations are traversed recursively and concatenated into one document.
- Newer versions (version associations) are surfaced above the note.
- Context, chain, and association tools appear below the note in a stacked layout.
- The note body is presented as a single continuous document without visible segment boundaries.
- Edit action opens a modal composer prefilled with the current post content.
- The palette is a darker, bluish mode with angular surfaces.
