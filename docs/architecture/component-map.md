# Component Map

## Components

- TCP Listener: accepts connections and spawns tasks.
- HTTP Parser: produces request structs with headers/body.
- Router: maps method/path to handler functions.
- Handlers: orchestrate validation and storage calls.
- Storage Adapter: executes SQL and maps rows to domain types.
- Auth Validator: verifies Google tokens and extracts user info.

## Boundaries

- Domain logic must remain pure and side-effect free.
- IO occurs only in adapters and server layer.
