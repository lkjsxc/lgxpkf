ALTER TABLE users
    ADD COLUMN IF NOT EXISTS account_note_id BYTEA REFERENCES notes(id);

CREATE INDEX IF NOT EXISTS users_account_note_idx ON users(account_note_id);
