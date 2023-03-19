-- Add migration script here
CREATE TABLE IF NOT EXISTS tokens
(
    id TEXT PRIMARY KEY,
    expired_at TIMESTAMP WITH TIME ZONE NOT NULL
);

INSERT INTO tokens
(id, expired_at)
VALUES
    ('LET_ME_IN', (CURRENT_TIMESTAMP + INTERVAL '15 minutes') AT TIME ZONE 'utc');
