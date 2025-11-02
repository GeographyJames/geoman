CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    owner integer NOT NULL REFERENCES app.users(id),
    name text NOT NULL UNIQUE,
    slug text NOT NULL UNIQUE,
    country app.country NOT NULL,
    status app.status NOT NULL DEFAULT 'active',
    visibility app.visibility NOT NULL DEFAULT 'public',
    
);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.projects
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();