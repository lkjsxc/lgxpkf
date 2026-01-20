# Server

## Responsibilities

- Bind to the configured address.
- Initialize shared application state.
- Serve HTTP requests with Actix Web.

## Connection Lifecycle

- Accept connection and hand off to Actix worker.
- Extract request data using Actix extractors.
- Dispatch to handler functions.
- Map handler results to HTTP responses.
