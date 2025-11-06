CREATE SCHEMA app;
CREATE TYPE app.status AS ENUM ('active', 'archived', 'deleted');
CREATE TYPE app.visibility AS ENUM ('private', 'team', 'public');

CREATE OR REPLACE FUNCTION update_last_updated()
RETURNS TRIGGER AS $$
DECLARE
    current_app_user INTEGER;
BEGIN
    -- Update timestamp
    NEW.last_updated = NOW();

    -- Try to get user from session variable
    BEGIN
        current_app_user := current_setting('app.current_user_id')::INTEGER;
    EXCEPTION WHEN OTHERS THEN
        RAISE EXCEPTION 'No user context set. Call SET app.current_user_id = ?'
            USING HINT = 'Use SET LOCAL app.current_user_id in transaction';
    END;

    -- Set last_updated_by from session variable
    NEW.last_updated_by := current_app_user;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE DOMAIN slug AS TEXT
    CHECK (
        VALUE ~ '^[a-z0-9]+(-[a-z0-9]+)*$'
        AND length(VALUE) BETWEEN 1 AND 100
    );

COMMENT ON DOMAIN slug IS 'URL-friendly slug: lowercase alphanumeric with hyphens as separators';