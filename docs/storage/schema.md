# Schema

## Tables

- users
- sessions
- notes
- associations
- follows

## Users Table

- user_id (uuid)
- google_sub (text)
- email (text)
- account_note_id (bytea, nullable)
- created_at (timestamptz)

## Notes Table

- id (bytea)
- value (bytea)
- created_at (timestamptz)
- author_id (uuid)

## Follows Table

- follower_id (uuid)
- followee_id (uuid)
- created_at (timestamptz)
