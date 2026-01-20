# Concurrency Model

## Runtime

- Actix Web runs on Tokio with worker threads.
- One task per request, with async database calls.
- Connection handling is managed by Actix.

## Back-Pressure

- Limit request body size via Actix payload settings.
- Use timeouts for downstream IO.
- Reject overload with 503 for readiness failures.
