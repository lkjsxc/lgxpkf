# Timeline UI

## Behavior

- Fetches GET /feed after authentication.
- Renders notes in reverse chronological order.
- Timeline uses a grid layout (three columns on desktop).
- Cards include inline reply and share actions alongside the Google account icon.
- Clicking the card body navigates to the note.
- Overly long notes are truncated to about three lines.
- Timeline metadata excludes the note id and uses the author email with icon.
- Notes with newer versions are hidden from the timeline.
- Auto-refresh runs after posting.
- Signed-out root view uses GET /notes/random for a sample timeline.
- Hover styling updates borders without moving cards.
