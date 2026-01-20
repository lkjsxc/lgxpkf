# Note View

## Route

- GET /{base32_id}

## Behavior

- Server renders the note content and metadata.
- Markdown is rendered into HTML for the main body.
- prev/next associations are traversed recursively and concatenated into one document.
- Newer versions (version associations) are surfaced above the note.
- Sections appear in this order: note, content, associations, chain, link note.
- Associations and chain lists use multi-column grids (about three columns on desktop).
- The note body is presented as a single continuous document without visible segment boundaries.
- Edit action opens a modal composer prefilled with the current post content.
- The palette is a darker, solid-color base with slightly reduced corner roundness.
- Card content enforces text wrapping to avoid overflow.
