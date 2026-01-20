# Component Map

## Components

- Actix HTTP Server: worker threads and connection handling.
- Actix Router: path/method mapping to handlers.
- Handlers: orchestrate validation and storage calls.
- Extractors: JSON, path, query, and auth token parsing.
- Storage Adapter: executes SQL and maps rows to domain types.
- Auth Validator: verifies Google tokens and extracts user info.

## Boundaries

- Domain logic must remain pure and side-effect free.
- IO occurs only in adapters and handlers.
