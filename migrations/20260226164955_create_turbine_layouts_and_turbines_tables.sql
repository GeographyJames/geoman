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
    hub_height_mm integer,
    rotor_diameter_mm integer,
    geom geometry(POINT) NOT NULL CHECK (ST_IsValid(geom)),
    UNIQUE (layout_id, turbine_number),
    CONSTRAINT duplicate_turbine_exclusion EXCLUDE USING gist (
        layout_id WITH =,
        st_buffer(st_transform(geom, 4326)::geography, 1) WITH &&)
);

CREATE INDEX idx_turbines_layout ON app.turbines(layout_id);


CREATE OR REPLACE FUNCTION app.check_turbine_layout_crs_consistency()
RETURNS trigger AS $$
DECLARE
    existing_srid integer;
BEGIN
    SELECT ST_SRID(geom)
      INTO existing_srid
      FROM app.turbines
     WHERE layout_id = NEW.layout_id
       AND id != NEW.id
     LIMIT 1;

    IF existing_srid IS NOT NULL AND existing_srid != ST_SRID(NEW.geom) THEN
        RAISE EXCEPTION
            'Turbine SRID % does not match existing layout SRID %',
            ST_SRID(NEW.geom), existing_srid
            USING ERRCODE = 'check_violation',
                  CONSTRAINT = 'turbine_layout_crs_consistency';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER turbine_layout_crs_consistency
BEFORE INSERT OR UPDATE OF geom ON app.turbines
FOR EACH ROW EXECUTE FUNCTION app.check_turbine_layout_crs_consistency();