# System Overview

## Layers

1. HTTP Server: raw TCP, request parsing, response writing.
2. Application Core: pure functions for routing and use-cases.
3. Storage: PostgreSQL access via async driver.
4. Auth: Google token verification and session handling.

## Data Flow Summary

Request -> Parse -> Route -> Validate -> Use-case -> Persist -> Response
