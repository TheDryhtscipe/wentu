-- Add pref_deadline column to wentus table
ALTER TABLE wentus ADD COLUMN pref_deadline TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() + interval '1 day');

-- Update the default to not be needed for future inserts
ALTER TABLE wentus ALTER COLUMN pref_deadline DROP DEFAULT;
