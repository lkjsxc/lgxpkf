# Request Parsing

## Parsing Steps

- Read request line.
- Parse headers with strict limits.
- Enforce maximum body size.
- Decode JSON body if applicable.

## Limits

- Header size and count capped.
- Body size capped at 1 MiB.
