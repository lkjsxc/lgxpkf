# Note

## Definition

A note is an immutable segment with the following fields:

- id: 32-byte unique identifier.
- value: user content, maximum 1024 bytes.
- created_at: timestamp when created.
- author: user information at creation time.

## Constraints

- All fields are immutable after creation.
- Value length must be validated before persistence.
- Posts larger than 1024 bytes are split into multiple notes and linked.

## Composition

- A post is a chain of notes connected by next/prev associations.
- The head note (no predecessor link) is the public anchor for the post.

## Serialization

- JSON with base32 encoded id.
- created_at as RFC 3339 timestamp.
