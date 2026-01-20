# Routing

## Strategy

- Use Actix Web routing with explicit handlers.
- Prefer static paths and explicit path/query extractors.
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
