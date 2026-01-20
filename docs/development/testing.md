# Testing

## Scope

- Unit tests for parsers and domain logic.
- Integration tests for HTTP and DB.
- Web assets compile successfully from TypeScript.

## Strategy

- Pure functions tested with small fixtures.
- Run tests in Docker Compose when possible.
- Run `npm run build:web` before local Rust builds.
