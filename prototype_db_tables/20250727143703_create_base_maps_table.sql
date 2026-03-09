CREATE TYPE WMS_DATASOURCE_TYPE AS ENUM ('xyz', 'wms');

  CREATE TABLE app.base_maps (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      data_provider_id INTEGER NOT NULL REFERENCES app.base_map_data_providers(id),
      name TEXT NOT NULL UNIQUE,
      slug TEXT NOT NULL UNIQUE,
      url TEXT NOT NULL,
      type WMS_DATASOURCE_TYPE NOT NULL,
      epsg_id INTEGER NOT NULL,
      default_main_map_base_map BOOLEAN NOT NULL DEFAULT false,
      default_overview_map_base_map BOOLEAN NOT NULL DEFAULT false,
      layers TEXT,
      authcfg_id TEXT,
      wmts_tile_matrix_set TEXT,
      CHECK (slug ~ '^[a-z0-9-]+$')
  );

  -- Create partial unique indexes to enforce single default constraint
  CREATE UNIQUE INDEX idx_single_default_main_map
      ON app.base_maps (default_main_map_base_map)
      WHERE default_main_map_base_map = true;

  CREATE UNIQUE INDEX idx_single_default_overview_map
      ON app.base_maps (default_overview_map_base_map)
      WHERE default_overview_map_base_map = true;