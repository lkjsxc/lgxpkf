# Timeline UI

## Behavior

- Fetches GET /feed after authentication.
- Renders notes in reverse chronological order.
- Timeline uses a grid layout (three columns on desktop).
- Entire cards are clickable to open a note (no inner button).
- Overly long notes are truncated to about three lines.
- Timeline metadata excludes the note id.
- Notes with newer versions are hidden from the timeline.
- Auto-refresh runs after posting.
