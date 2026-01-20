# Functional Requirements

## Authentication

- Users can log in with Google accounts.
- Access tokens are validated on each authenticated request.

## Notes

- Users can create notes with immutable content.
- Posts can exceed 1024 bytes by chaining multiple notes.
- Notes are retrievable at /{id} for HTML view and /notes/{id} for JSON.
- Notes list is queryable by author and time range.

## Associations

- Notes can be linked by a kind and two note IDs.
- Associations can represent ordering, versioning, replies, and hierarchy.
- Version associations connect older notes to newer replacements.

## Follows

- Users can follow and unfollow other users.
- Followers and following lists are queryable.

## Feed

- Authenticated users can fetch a feed of notes from themselves and followees.
- Feed returns chain heads (notes without a prev/next predecessor).
- Feed supports time range filtering and bounded limits.

## Users

- User profiles are retrievable by user ID.

## Web UI

- The server serves an HTML page for note posting, timeline, and associations.
- The UI authenticates via Google and uses the JSON API.
- Related notes and version tools are displayed on the note view, with citations.
- Version associations label old vs newer posts and appear under Associations.
- The root page explains lgxpkf when signed out and hides the Post button.
- The note view includes follow/unfollow controls for the author.
- Google sign-in renders with a white outline theme and avoids flicker on load.
- Users must accept Terms, Privacy, and Guideline before sign-in can proceed.
- Policy pages are available at /terms, /privacy, and /guideline.
- The top bar is identical across pages and shows either Sign in or the account menu.
- Account menu provides My posts and Sign out.
- A sign-in page exists at /signin and handles policy consent.
- Signed-out root view shows a random timeline.

## APIs

- HTTP APIs for notes, associations, and user session info.
- Errors are returned with stable codes and messages.
