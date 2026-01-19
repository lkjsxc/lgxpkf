CREATE TABLE IF NOT EXISTS follows (
    follower_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    followee_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (follower_id <> followee_id),
    PRIMARY KEY (follower_id, followee_id)
);

CREATE INDEX IF NOT EXISTS follows_follower_idx ON follows(follower_id);
CREATE INDEX IF NOT EXISTS follows_followee_idx ON follows(followee_id);
