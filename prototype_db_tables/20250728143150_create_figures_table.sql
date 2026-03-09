-- Add migration script here
CREATE TYPE figure_status AS ENUM ('active', 'archived', 'deleted');

CREATE TABLE app.figures (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parent_id INTEGER REFERENCES app.figures(id),
    project_id INTEGER NOT NULL REFERENCES app.projects(id),
    main_map_base_map_id INTEGER REFERENCES app.base_maps(id),
    overview_map_base_map_id INTEGER REFERENCES app.base_maps(id),
    qgis_project_uuid UUID NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by INTEGER NOT NULL REFERENCES app.users(id),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_by INTEGER NOT NULL REFERENCES app.users(id),
    status  FIGURE_STATUS NOT NULL DEFAULT 'active'::figure_status,
    page_width_mm INTEGER NOT NULL,
    page_height_mm INTEGER NOT NULL,
    margin_mm INTEGER NOT NULL,
    legend_width_mm INTEGER NOT NULL,
    scale INTEGER NOT NULL,
    srid INTEGER NOT NULL,
    properties JSON NOT NULL
);

  -- Create trigger
  CREATE TRIGGER update_last_updated_trigger
      BEFORE UPDATE ON app.figures
      FOR EACH ROW
      EXECUTE FUNCTION update_last_updated();