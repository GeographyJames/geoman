-- Add new columns
ALTER TABLE app.figure_layers ADD COLUMN site_boundary_id INTEGER REFERENCES app.site_boundaries(id);
ALTER TABLE app.figure_layers ADD COLUMN turbine_layout_id INTEGER REFERENCES app.turbine_layouts(id);
ALTER TABLE app.figure_layers ADD COLUMN project_layer_source JSONB;

-- Migrate data from source column to new columns
UPDATE app.figure_layers
SET site_boundary_id = (source ->> 'SiteBoundary')::int
WHERE source ? 'SiteBoundary';

UPDATE app.figure_layers
SET turbine_layout_id = (source ->> 'TurbineLayout')::int
WHERE source ? 'TurbineLayout';

UPDATE app.figure_layers
SET project_layer_source = source -> 'PgTable'
WHERE source ? 'PgTable';

-- Add constraint to ensure only one datasource type is set
ALTER TABLE app.figure_layers
ADD CONSTRAINT check_single_datasource
CHECK (
  (site_boundary_id IS NOT NULL AND turbine_layout_id IS NULL AND project_layer_source IS NULL) OR
  (site_boundary_id IS NULL AND turbine_layout_id IS NOT NULL AND project_layer_source IS NULL) OR
  (site_boundary_id IS NULL AND turbine_layout_id IS NULL AND project_layer_source IS NOT NULL)
);
