CREATE UNIQUE INDEX IF NOT EXISTS associations_version_from_idx
    ON associations(from_id)
    WHERE kind = 'version';
