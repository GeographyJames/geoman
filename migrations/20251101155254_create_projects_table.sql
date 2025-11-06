CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    slug slug NOT NULL UNIQUE,
    status app.status NOT NULL DEFAULT 'active',
    visibility app.visibility NOT NULL DEFAULT 'public',
    owner integer NOT NULL REFERENCES app.users(id),
    
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now()


    -- country_code CHAR(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    -- subdivision_code VARCHAR(6) CHECK (subdivision_code ~ '^[A-Z]{2}-[A-Z0-9]{1,3}$'),
    -- srid integer NOT NULL REFERENCES spatial_ref_sys(srid),

);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.projects
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();