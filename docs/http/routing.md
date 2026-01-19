# Routing

## Strategy

- Match by method and path.
- Keep a static routing table.
- Avoid regex or dynamic routing for performance.

## Examples

- GET /notes/{id}
- POST /notes
- GET /
- GET /feed
- POST /follows
- GET /users/{user_id}
