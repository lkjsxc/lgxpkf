# Routing

## Strategy

- Use the custom HTTP router with explicit handlers.
- Prefer static paths and clear parameter extraction.
- Keep handler modules small and functional.

## Examples

- GET /{id}
- GET /notes/{id}
- POST /notes
- GET /
- GET /feed
- POST /account/note
- POST /follows
- GET /users/{user_id}
- GET /notes/{id}/related
