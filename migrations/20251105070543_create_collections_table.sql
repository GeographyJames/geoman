CREATE TABLE app.collections (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_id integer REFERENCES app.projects(id),
    title text NOT NULL,
    slug text NOT NULL CHECK (slug ~ '[a-z]' AND slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    description TEXT,
    status app.status NOT NULL DEFAULT 'ACTIVE',
    geometry_type geometry_type NOT NULL,
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT project_collections_title_key UNIQUE (title),
    CONSTRAINT project_collections_slug_key UNIQUE (slug)
);



INSERT INTO app.collections (
    id,
    title,
    slug,
    geometry_type,
    added_by,
    last_updated_by,
    description
) OVERRIDING SYSTEM VALUE VALUES (
    0,
    'site boundaries',
    'site-boundaries',
    'MULTIPOLYGON',
   0,
   0,
   'the site boundaries'
);