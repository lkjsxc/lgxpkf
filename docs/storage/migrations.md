# Migrations

## Strategy

- Versioned SQL migrations.
- Immutable migrations once applied.
- Run on startup when configured.
- Container image includes `/app/db/migrations` for runtime execution.

## Recent Additions

- users.account_note_id column (0003_account_note.sql).
- Version uniqueness index for associations (0004_version_unique.sql).
