# Server

## Responsibilities

- Listen on TCP.
- Accept connections.
- Spawn tasks for request handling.

## Connection Lifecycle

- Read request bytes.
- Parse HTTP request line and headers.
- Dispatch to router.
- Write response.
