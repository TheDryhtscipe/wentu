-- Add timezone support for time slot configuration
ALTER TABLE wentus ADD COLUMN timezone VARCHAR(100);

COMMENT ON COLUMN wentus.timezone IS 'IANA timezone name (e.g., Europe/London). NULL for legacy full-day events.';
