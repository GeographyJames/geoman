CREATE TABLE app.turbine_layouts (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_id integer NOT NULL REFERENCES app.projects(id),
    name text NOT NULL,
    is_primary boolean NOT NULL DEFAULT false,
    status app.status NOT NULL DEFAULT 'ACTIVE',
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),

    CHECK (NOT (is_primary AND status IN ('DELETED', 'ARCHIVED')))
);

CREATE UNIQUE INDEX idx_one_primary_layout_per_project
ON app.turbine_layouts(project_id)
WHERE is_primary = true;

CREATE INDEX idx_turbine_layouts_project ON app.turbine_layouts(project_id);
CREATE INDEX idx_turbine_layouts_status ON app.turbine_layouts(status);

CREATE TABLE app.turbines (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    layout_id integer NOT NULL REFERENCES app.turbine_layouts(id),
    turbine_number integer NOT NULL,
    hub_height_mm integer NOT NULL,
    rotor_diameter_mm integer NOT NULL,
    geom geometry(POINT) NOT NULL CHECK (ST_IsValid(geom)),
    UNIQUE (layout_id, turbine_number),
    CONSTRAINT duplicate_turbine_exclusion EXCLUDE USING gist (
        layout_id WITH =,
        st_buffer(st_transform(geom, 4326)::geography, 1) WITH &&)
);

CREATE INDEX idx_turbines_layout ON app.turbines(layout_id);
