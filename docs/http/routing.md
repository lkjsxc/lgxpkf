# Routing

## Strategy

- Use Actix Web routing with explicit handlers.
- Prefer static paths and clear parameter extraction.
- Keep handler modules small and functional.

## Examples

- GET /notes/{id}
- POST /notes
- GET /
- GET /feed
- POST /follows
- GET /users/{user_id}
- GET /notes/{id}/related
