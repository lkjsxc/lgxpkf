# Indexing

## Notes

- Primary key on id.
- Index on author_id and created_at.

## Users

- Index on account_note_id.

## Associations

- Composite index on (from_id, to_id).
- Index on kind.
- Unique partial index on from_id for kind = 'version'.

## Follows

- Composite primary key on (follower_id, followee_id).
- Index on follower_id.
- Index on followee_id.
