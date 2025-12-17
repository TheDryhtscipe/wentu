-- Create enum for wentu status
CREATE TYPE wentu_status AS ENUM ('open', 'closed', 'expired');

-- Wentus table
CREATE TABLE wentus (
    id UUID PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    creator_name VARCHAR(100) NOT NULL,
    creator_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status wentu_status NOT NULL DEFAULT 'open'
);

CREATE INDEX idx_wentus_slug ON wentus(slug);
CREATE INDEX idx_wentus_expires_at ON wentus(expires_at);

-- Date ranges table
CREATE TABLE date_ranges (
    id UUID PRIMARY KEY,
    wentu_id UUID NOT NULL REFERENCES wentus(id) ON DELETE CASCADE,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    label VARCHAR(255) NOT NULL,
    sort_order INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_date_ranges_wentu_id ON date_ranges(wentu_id);

-- Participants table
CREATE TABLE participants (
    id UUID PRIMARY KEY,
    wentu_id UUID NOT NULL REFERENCES wentus(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    participant_key VARCHAR(255) NOT NULL,
    is_creator BOOLEAN NOT NULL DEFAULT FALSE,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_participants_wentu_id ON participants(wentu_id);
CREATE INDEX idx_participants_key ON participants(participant_key);

-- Rankings table (for STV preferences)
CREATE TABLE rankings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    participant_id UUID NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
    date_option_id UUID NOT NULL REFERENCES date_ranges(id) ON DELETE CASCADE,
    preference_order INT NOT NULL,
    UNIQUE(participant_id, date_option_id)
);

CREATE INDEX idx_rankings_participant_id ON rankings(participant_id);
CREATE INDEX idx_rankings_date_option_id ON rankings(date_option_id);
