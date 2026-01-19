# Concurrency Model

## Runtime

- Tokio multi-threaded runtime.
- One task per connection.
- Bounded work queues for heavy operations.

## Back-Pressure

- Limit request body size early.
- Use timeouts for read/write operations.
- Reject overload with 503.
