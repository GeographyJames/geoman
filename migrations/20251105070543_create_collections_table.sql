CREATE TABLE app.collections (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title text NOT NULL,
    description TEXT,
    geometry_type geometry_type NOT NULL,
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT project_collections_title_key UNIQUE (title)
);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.collections
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();

INSERT INTO app.collections (
    title,
    geometry_type,
    added_by,
    last_updated_by
) VALUES (
    'site boundaries',
    'MULTIPOLYGON',
   0,
   0
);