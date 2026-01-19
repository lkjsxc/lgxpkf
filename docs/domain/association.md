# Association

## Definition

An association links two notes with a single semantic kind token.

- kind: single-token string.
- from_id: source note id.
- to_id: target note id.
- created_at: timestamp.

## Semantics

- next / prev: ordering links used to chain multi-note posts.
- version: replacement link from an older note to its newer version.
- new / old: optional replacement markers for external tooling.
- parent / child: hierarchy links.
- reply: direct reply or quote.
- link: generic association for loose references.
- next / prev are traversed recursively on the note page to concatenate a single document.
