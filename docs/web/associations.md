# Association UI

## Behavior

- The home UI no longer exposes an association form.
- The note page exposes an association form for linking other notes.
- Associations are still created via POST /associations for automation or tooling.
- The link form is only enabled for the author of the current note.
- Target accepts a base32 note id or full note URL.
- Structural kinds (parent/child/next/prev) require notes by the same author.
- next and prev are used to build recursive note chains on the note page.
- Association cards wrap text and use a multi-column grid layout on desktop.
- Association cards show citations (note IDs) and distinguish newer vs older versions.
- Reply associations label direction (Reply to / Reply from).
