CREATE TABLE app.projects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    status app.status NOT NULL DEFAULT 'ACTIVE',
    visibility app.visibility NOT NULL DEFAULT 'PUBLIC',
    country_code character(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    owner integer NOT NULL REFERENCES app.users(id),
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    crs_srid integer REFERENCES public.spatial_ref_sys(srid)
);

CREATE INDEX idx_country_code ON app.projects(country_code);
CREATE INDEX idx_project_owner ON app.projects(owner);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.projects
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();