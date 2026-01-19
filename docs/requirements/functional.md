# Functional Requirements

## Authentication

- Users can log in with Google accounts.
- Access tokens are validated on each authenticated request.

## Notes

- Users can create notes with immutable content.
- Notes are retrievable at /{id} for HTML view and /notes/{id} for JSON.
- Notes list is queryable by author and time range.

## Associations

- Notes can be linked by a kind and two note IDs.
- Associations can represent ordering, versioning, replies, and hierarchy.

## Follows

- Users can follow and unfollow other users.
- Followers and following lists are queryable.

## Feed

- Authenticated users can fetch a feed of notes from themselves and followees.
- Feed supports time range filtering and bounded limits.

## Users

- User profiles are retrievable by user ID.

## Web UI

- The server serves an HTML page for note posting, timeline, and associations.
- The UI authenticates via Google and uses the JSON API.
- Related notes are displayed for a selected note and the note view.

## APIs

- HTTP APIs for notes, associations, and user session info.
- Errors are returned with stable codes and messages.
