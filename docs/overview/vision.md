# Vision

## Purpose

Build a minimal, high-performance SNS server using Rust and tokio, with a custom HTTP server and PostgreSQL storage.

## Core Principles

- Functional programming style with explicit data flow.
- Strong immutability for domain objects.
- Predictable latency and back-pressure handling.
- Operational clarity for deployment and maintenance.

## Scope

- Authentication via Google account.
- Notes with immutable content and identifiers.
- Note associations for versioning, linking, and replies.
- Follow relationships and a feed timeline.
- Base32 URL addressing for notes.
