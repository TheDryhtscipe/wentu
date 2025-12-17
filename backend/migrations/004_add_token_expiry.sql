-- Add token expiration to participants
ALTER TABLE participants
    ADD COLUMN token_expires_at TIMESTAMP WITH TIME ZONE;

-- Backfill existing participants with a 7-day window
UPDATE participants
SET token_expires_at = NOW() + INTERVAL '7 days'
WHERE token_expires_at IS NULL;

-- Enforce non-null for future rows
ALTER TABLE participants
    ALTER COLUMN token_expires_at SET NOT NULL;

-- Index for cleanup and lookups
CREATE INDEX idx_participants_token_expires_at ON participants(token_expires_at);
