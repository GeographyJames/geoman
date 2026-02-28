CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    team_id integer NOT NULL REFERENCES app.teams(id),
    search_area_id integer REFERENCES app.search_areas(id),
    search_site_name text,
    name text NOT NULL,
    slug text NOT NULL CHECK (slug ~ '[a-z]' AND slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    status app.status NOT NULL DEFAULT 'ACTIVE',
    visibility app.visibility NOT NULL DEFAULT 'PUBLIC',
    owner integer NOT NULL REFERENCES app.users(id),
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    crs_srid integer,
    CONSTRAINT projects_name_key UNIQUE (name),
    CONSTRAINT projects_slug_key UNIQUE (slug),
    CONSTRAINT projects_codename_search_area_id_key UNIQUE (search_site_name, search_area_id),
    CONSTRAINT search_site_name_not_null_when_search_area_id_not_null CHECK (search_area_id IS NULL OR search_site_name IS NOT NULL)
);

CREATE INDEX idx_project_owner ON app.projects(owner);
CREATE INDEX idx_projects_search_area_id ON app.projects(search_area_id);
CREATE INDEX idx_team_id ON app.projects(team_id)
