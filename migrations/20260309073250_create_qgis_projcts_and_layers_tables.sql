CREATE TABLE public.layer_styles (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    f_table_catalog character varying COLLATE pg_catalog."default",
    f_table_schema character varying COLLATE pg_catalog."default",
    f_table_name character varying COLLATE pg_catalog."default",
    f_geometry_column character varying COLLATE pg_catalog."default",
    stylename text NOT NULL UNIQUE COLLATE pg_catalog."default",
    styleqml xml,
    stylesld xml,
    useasdefault boolean,
    description text COLLATE pg_catalog."default",
    owner character varying(63) NOT NULL COLLATE pg_catalog."default" DEFAULT CURRENT_USER,
    ui xml,
    update_time timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type character varying COLLATE pg_catalog."default"
);

CREATE TABLE app.figures (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parent_id INTEGER REFERENCES app.figures(id),
    project_id INTEGER NOT NULL REFERENCES app.projects(id),
    main_map_base_map_id INTEGER REFERENCES app.data_provider_layers(id),
    overview_map_base_map_id INTEGER REFERENCES app.data_provider_layers(id),
    qgis_project_uuid UUID NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by INTEGER NOT NULL REFERENCES app.users(id),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_by INTEGER NOT NULL REFERENCES app.users(id),
    status app.status NOT NULL DEFAULT 'ACTIVE',
    page_width_mm INTEGER NOT NULL,
    page_height_mm INTEGER NOT NULL,
    margin_mm INTEGER NOT NULL,
    legend_width_mm INTEGER NOT NULL,
    scale INTEGER NOT NULL,
    srid INTEGER NOT NULL,
    properties JSONB NOT NULL
);

CREATE TABLE app.figure_layers (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    figure_id INTEGER NOT NULL REFERENCES app.figures(id),
    style_id INTEGER REFERENCES public.layer_styles(id),
    name TEXT NOT NULL,
    layer_order INTEGER NOT NULL,
    properties JSONB NOT NULL,
    site_boundary_id INTEGER REFERENCES app.project_features(id),
    turbine_layout_id INTEGER REFERENCES app.turbine_layouts(id),
    project_layer_source JSONB,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by INTEGER NOT NULL REFERENCES app.users(id),
    CONSTRAINT unique_layer_order_per_figure UNIQUE (figure_id, layer_order),
    CONSTRAINT unique_layer_name_per_figure UNIQUE (figure_id, name),
    CONSTRAINT check_single_datasource CHECK (
        (site_boundary_id IS NOT NULL AND turbine_layout_id IS NULL AND project_layer_source IS NULL) OR
        (site_boundary_id IS NULL AND turbine_layout_id IS NOT NULL AND project_layer_source IS NULL) OR
        (site_boundary_id IS NULL AND turbine_layout_id IS NULL AND project_layer_source IS NOT NULL)
    )
);

CREATE INDEX ON app.figure_layers(figure_id);
