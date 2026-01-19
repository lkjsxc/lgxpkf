# Follows API

## POST /follows

- Auth required.
- Body:
  - followee_id (uuid)
- Response: 201 with Follow.

## DELETE /follows

- Auth required.
- Body:
  - followee_id (uuid)
- Response: 200 with status payload.

## GET /follows

- Query:
  - user (uuid)
  - direction (followers|following)
- Response: 200 with edges list.
