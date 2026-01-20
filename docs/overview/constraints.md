# Constraints

## Technology

- Rust, tokio, PostgreSQL.
- Custom HTTP server and router (no web framework).
- Web UI served as static HTML from the same server.

## Documentation

- One README.md per directory (table of contents).
- Recursive structure with multiple files in each directory.
- Each documentation file under 300 lines.

## Implementation

- Source files under 200 lines.
- Functional style and performance-oriented patterns.
- No backward compatibility requirements.
- Distroless runtime images with baked migrations.
- Images published automatically to GHCR.
