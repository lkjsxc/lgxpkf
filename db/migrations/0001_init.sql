CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    google_sub TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sessions (
    token TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS sessions_user_id_idx ON sessions(user_id);
CREATE INDEX IF NOT EXISTS sessions_expires_idx ON sessions(expires_at);

CREATE TABLE IF NOT EXISTS notes (
    id BYTEA PRIMARY KEY,
    value BYTEA NOT NULL CHECK (octet_length(value) <= 1024),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    author_id UUID NOT NULL REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS notes_author_created_idx ON notes(author_id, created_at);

CREATE TABLE IF NOT EXISTS associations (
    id UUID PRIMARY KEY,
    kind TEXT NOT NULL,
    from_id BYTEA NOT NULL REFERENCES notes(id),
    to_id BYTEA NOT NULL REFERENCES notes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS associations_from_to_idx ON associations(from_id, to_id);
CREATE INDEX IF NOT EXISTS associations_kind_idx ON associations(kind);
