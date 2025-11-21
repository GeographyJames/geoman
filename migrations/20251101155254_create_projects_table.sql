CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    status app.status NOT NULL DEFAULT 'ACTIVE',
    visibility app.visibility NOT NULL DEFAULT 'PUBLIC',
    owner integer NOT NULL REFERENCES app.users(id),
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now()
);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.projects
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();