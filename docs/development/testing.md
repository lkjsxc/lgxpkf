# Testing

## Scope

- Unit tests for domain logic and helpers.
- Integration tests for Actix handlers and DB access.
- Web assets compile successfully from TypeScript.

## Strategy

- Pure functions tested with small fixtures.
- Run tests in Docker Compose when possible.
- Run `npm run build:web` before local Rust builds when assets are needed.
