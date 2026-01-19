# Performance

## Goals

- Low allocation in hot paths.
- Bounded buffering for IO.

## Techniques

- Reuse buffers where safe.
- Keep payload parsing linear time.
