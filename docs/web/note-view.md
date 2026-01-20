# Note View

## Route

- GET /{base32_id}

## Behavior

- Server renders the note content and metadata.
- Markdown is rendered into HTML for the main body.
- prev/next associations are traversed recursively and concatenated into one document.
- Version associations are shown within Associations and a newer-version panel between Note and Content.
- Sections appear in this order: note, version panel (if present), content, associations, chain, link note.
- Associations and chain lists use multi-column grids (about three columns on desktop).
- Association cards include citations (note IDs) and version cards label newer vs older posts.
- The note body is presented as a single continuous document without visible segment boundaries.
- Action row at the bottom of the note card contains Copy link, Copy JSON, Follow, Edit.
- Edit action opens a modal composer prefilled with the current post content (except account bootstrap notes).
- Link note form supports kinds: link, reply, quote, parent, child, next, prev.
- The palette is a darker, solid-color base with slightly reduced corner roundness.
- Card content enforces text wrapping to avoid overflow.
- Reply associations are labeled as Reply to or Reply from depending on direction.
