# Threat Model

## Threats

- Token theft and replay.
- Payload abuse and oversized bodies.
- SQL injection.
- Enumeration of note IDs.

## Mitigations

- Token validation and expiry checks.
- Strict body size limits.
- Prepared statements and parameter binding.
- 32-byte IDs with high entropy.
