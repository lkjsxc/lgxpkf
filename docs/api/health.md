# Health

## Endpoints

- `GET /health` returns a lightweight process check.
- `GET /ready` validates database connectivity.

## Responses

- `200` with `{"status":"ok"}` or `{"status":"ready"}`.
- `503` when readiness checks fail.
