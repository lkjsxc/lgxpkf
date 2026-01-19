# PostgreSQL

## Driver

- Use async PostgreSQL driver.
- Connection pool managed by tokio tasks.

## Transactions

- Use explicit transactions for multi-table writes.
- Ensure idempotency for retries where safe.
