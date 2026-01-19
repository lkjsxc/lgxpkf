# Association

## Definition

An association links two notes with a single semantic kind token.

- kind: single-token string.
- from_id: source note id.
- to_id: target note id.
- created_at: timestamp.

## Semantics

- next / prev: ordering links.
- new / old: replacement links.
- parent / child: hierarchy links.
- reply: direct reply or quote.
- version: new version of a note.
- next / prev are traversed recursively on the note page to concatenate a single document.
