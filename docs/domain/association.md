# Association

## Definition

An association links two notes with a single semantic kind token.

- kind: single-token string.
- from_id: source note id.
- to_id: target note id.
- created_at: timestamp.

## Semantics

- next / prev: ordering links used to chain multi-note posts.
- version: replacement link from an older note to its newer version (one newer version per note).
- new / old: optional replacement markers for external tooling.
- parent / child: hierarchy links.
- reply: direct reply or quote.
- link: generic association for loose references.
- author: links a user account note to a note it authored.
- next / prev are traversed recursively on the note page to concatenate a single document.

## Ownership Rules

- User-created associations require the caller to own the source note (from_id).
- Cross-author associations are limited to link, reply, quote.
- parent/child/next/prev require both notes to share the same author.
- System-only kinds (author, version) are created by the backend.
- Version associations always connect two notes by the same author.
