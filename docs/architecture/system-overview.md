# System Overview

## Layers

1. Actix Web: HTTP server, routing, extractors, response mapping.
2. Application Core: pure functions for use-cases and validation.
3. Storage: PostgreSQL access via async driver and pool.
4. Auth: Google token verification and session handling.

## Data Flow Summary

Request -> Extract -> Validate -> Use-case -> Persist -> Response
