# Request Parsing

## Parsing Steps

- Use Actix extractors for path, query, and JSON bodies.
- Apply JSON payload limits on the App configuration.
- Validate schemas in handler-layer helpers.

## Limits

- Payload size capped via Actix JsonConfig.
- Query and path parsing rely on typed extractors.
