CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    team_id integer NOT NULL REFERENCES app.teams(id),
    search_area_id integer REFERENCES app.search_areas(id),
    search_site_name text,
    name text NOT NULL UNIQUE CHECK (name ~ '[A-Za-z]'),
    slug text NOT NULL UNIQUE CHECK (slug ~ '[a-z]' AND slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    status app.status NOT NULL DEFAULT 'ACTIVE',
    visibility app.visibility NOT NULL DEFAULT 'PUBLIC',
    country_code character(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    owner integer NOT NULL REFERENCES app.users(id),
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    crs_srid integer REFERENCES public.spatial_ref_sys(srid),
    CONSTRAINT projects_codename_search_area_id_key UNIQUE (search_site_name, search_area_id),
    CONSTRAINT search_site_name_not_null_when_search_area_id_not_null CHECK (search_area_id IS NULL OR search_site_name IS NOT NULL)
);

CREATE INDEX idx_country_code ON app.projects(country_code);
CREATE INDEX idx_project_owner ON app.projects(owner);
CREATE INDEX idx_projects_search_area_id ON app.projects(search_area_id);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.projects
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();