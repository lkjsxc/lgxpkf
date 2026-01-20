# Runbooks

## Startup

- Ensure DB is healthy.
- Start server with required env vars.
- Confirm `GET /ready` returns 200 before routing traffic.

## Incident

- Check logs for error codes.
- Validate DB connections and pool usage.
