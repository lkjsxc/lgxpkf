# Schema

## Tables

- users
- sessions
- notes
- associations
- follows

## Notes Table

- id (bytea)
- value (bytea)
- created_at (timestamptz)
- author_id (uuid)

## Follows Table

- follower_id (uuid)
- followee_id (uuid)
- created_at (timestamptz)
