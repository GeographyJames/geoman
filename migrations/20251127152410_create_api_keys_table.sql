-- Create api_keys table for QGIS and other machine client authentication
CREATE TABLE app.api_keys (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL REFERENCES app.users(id) ON DELETE CASCADE,
    key_hash TEXT NOT NULL UNIQUE, -- Hashed API key (never store plaintext)
    name TEXT, -- Optional user-friendly name like "My QGIS Desktop"
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ, -- Track when key was last used
    revoked BOOLEAN NOT NULL DEFAULT FALSE -- Soft delete for audit trail
);

-- Critical index for fast lookup by hash during authentication
CREATE UNIQUE INDEX idx_api_keys_key_hash ON app.api_keys(key_hash);

-- Index for listing a user's keys
CREATE INDEX idx_api_keys_user_id ON app.api_keys(user_id);

-- Index for finding active (non-revoked) keys
CREATE INDEX idx_api_keys_active ON app.api_keys(user_id, revoked) WHERE revoked = FALSE;
