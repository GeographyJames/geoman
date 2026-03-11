CREATE TABLE public.qgis_projects (
    name TEXT COLLATE pg_catalog."default" NOT NULL,
    metadata JSONB NOT NULL,
    content BYTEA NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    figure_id INTEGER NOT NULL REFERENCES app.figures(id),
    low_res BOOLEAN NOT NULL,
    CONSTRAINT qgis_projects_pkey PRIMARY KEY (name)
);