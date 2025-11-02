CREATE SCHEMA app;
CREATE TYPE app.status AS ENUM ('active', 'archived', 'deleted');
CREATE TYPE app.visibility AS ENUM ('private', 'team', 'public');
CREATE TYPE app.country AS ENUM ('Scotland', 'England', 'Wales', 'Northern Ireland', 'Ireland');

CREATE OR REPLACE FUNCTION update_last_updated()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;