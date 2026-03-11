CREATE TYPE app.data_provider_service_type AS ENUM (
    'ImageWMS',
    'TileWMS',
    'WMTS',
    'WFS',
    'ArcGISRest',
    'MVT',
    'OGCAPIFeatures',
    'XYZ'
);

CREATE TYPE app.layer_category AS ENUM (
    'overlay',
    'basemap'
);

CREATE TABLE app.data_providers (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text UNIQUE NOT NULL,
    country_code char(2),      -- ISO 3166-1 alpha-2, NULL = global
    subdivision varchar(10),   -- ISO 3166-2, e.g. 'GB-SCT', NULL = whole country
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    copyright_text TEXT
);


CREATE INDEX idx_data_providers_country ON app.data_providers(country_code, subdivision);


CREATE TABLE app.data_provider_services (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    provider_id integer NOT NULL REFERENCES app.data_providers(id) ON DELETE CASCADE,
    name text NOT NULL,
    service_type app.data_provider_service_type NOT NULL,
    base_url text,

    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_data_provider_services_provider ON app.data_provider_services(provider_id);



CREATE TABLE app.data_provider_layers (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    service_id integer NOT NULL REFERENCES app.data_provider_services(id) ON DELETE CASCADE,
    name text NOT NULL UNIQUE,
    slug text NOT NULL UNIQUE CHECK (slug ~ '[a-z]' AND slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    abbreviation text,         -- short label for map legend, e.g. 'SSSI', 'AONB'
    -- Flexible source config — shape varies by service type, e.g.:
    -- ImageWMS/TileWMS: {"layers": "inspire-nrw:NRW_SSSI", "styles": "", "format": "image/png"}
    -- WMTS:             {"layer": "Road_3857", "matrix_set": "EPSG:3857", "format": "image/png"}
    -- ArcGISRest:       {"layer_id": 0}
    -- WFS/OGCAPIFeatures: {"type_name": "inspire-nrw:NRW_SSSI"}
    -- MVT:              {"layer": "streets-v8"}
    -- XYZ:              {}
    source jsonb,
    category app.layer_category NOT NULL DEFAULT 'overlay',
    description text,
    enabled_geoman boolean NOT NULL DEFAULT false,
    enabled_figure_tool NOT NULL DEFAULT true,
    style_config jsonb NOT NULL DEFAULT '{}',    -- SLD XML string: {"sld": "<StyledLayerDescriptor>..."}
    display_options jsonb NOT NULL DEFAULT '{}', -- e.g. {"opacity": 0.8, "min_zoom": 10, "max_zoom": 18}
    country_code char(2),      -- overrides provider if set
    subdivision varchar(10),   -- overrides provider if set
    sort_order integer NOT NULL DEFAULT 0,
    figure_default_main_map_base_map BOOLEAN NOT NULL DEFAULT false,
    figure_default_overview_map_base_map BOOLEAN NOT NULL DEFAULT false,

    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_data_provider_layers_service ON app.data_provider_layers(service_id);

CREATE INDEX idx_data_provider_layers_enabled ON app.data_provider_layers(enabled) WHERE enabled = true;
CREATE INDEX idx_data_provider_layers_category ON app.data_provider_layers(category);
  -- Create partial unique indexes to enforce single default constraint
  CREATE UNIQUE INDEX idx_single_default_main_map
      ON app.data_provider_layers (figure_default_main_map_base_map)
      WHERE figure_default_main_map_base_map = true;

  CREATE UNIQUE INDEX idx_single_default_overview_map
      ON app.data_provider_layers (figure_default_overview_map_base_map)
      WHERE figure_default_overview_map_base_map = true;

CREATE UNIQUE INDEX uniq_provider_service
ON app.data_provider_services(provider_id, service_type);