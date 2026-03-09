DROP TABLE IF EXISTS app.base_maps CASCADE;
DROP TYPE IF EXISTS WMS_DATASOURCE_TYPE;



  CREATE TABLE app.base_maps (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      data_provider_id INTEGER NOT NULL REFERENCES app.base_map_data_providers(id),
      name TEXT NOT NULL UNIQUE,
      slug TEXT NOT NULL UNIQUE,
      datasource JSONB,
      default_main_map_base_map BOOLEAN NOT NULL DEFAULT false,
      default_overview_map_base_map BOOLEAN NOT NULL DEFAULT false,


      CHECK (slug ~ '^[a-z0-9-]+$')
  );

  -- Create partial unique indexes to enforce single default constraint
  CREATE UNIQUE INDEX idx_single_default_main_map
      ON app.base_maps (default_main_map_base_map)
      WHERE default_main_map_base_map = true;

  CREATE UNIQUE INDEX idx_single_default_overview_map
      ON app.base_maps (default_overview_map_base_map)
      WHERE default_overview_map_base_map = true;