CREATE SCHEMA qgis

CREATE TABLE qgis.qgis_projects (
    name TEXT COLLATE pg_catalog."default" NOT NULL,
    metadata jsonb NOT NULL,
    content bytea NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    figure_id INTEGER NOT NULL REFERENCES app.figures(id),
    low_res BOOLEAN NOT NULL,
    CONSTRAINT qgis_projects_pkey PRIMARY KEY (name)
);