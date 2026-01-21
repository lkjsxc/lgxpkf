# Threat Model

## Threats

- Token theft and replay.
- Payload abuse and oversized bodies.
- SQL injection.
- Enumeration of note IDs.
- Association tampering (forged replies/versions or hiding posts).

## Mitigations

- Token validation and expiry checks.
- Strict body size limits.
- Prepared statements and parameter binding.
- 32-byte IDs with high entropy.
- Ownership checks for association creation and version edits.
- System-only kinds are rejected on the public association endpoint.
