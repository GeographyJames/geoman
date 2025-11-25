CREATE TABLE IF NOT EXISTS gis_data.uk_countries
(
    gid integer NOT NULL DEFAULT nextval('gis_data.uk_countries_gid_seq'::regclass),
    ctry21cd character varying(9) COLLATE pg_catalog."default",
    ctry21nm character varying(17) COLLATE pg_catalog."default",
    ctry21nmw character varying(17) COLLATE pg_catalog."default",
    bng_e double precision,
    bng_n double precision,
    "long" double precision,
    lat double precision,
    globalid character varying(38) COLLATE pg_catalog."default",
    geom geometry(MultiPolygon,27700),
    CONSTRAINT uk_countries_pkey PRIMARY KEY (gid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS gis_data.uk_countries
    OWNER to app_local;

COMMENT ON TABLE gis_data.uk_countries
    IS 'UK country boundaries from December 2021';
-- Index: uk_countries_geom_idx

-- DROP INDEX IF EXISTS gis_data.uk_countries_geom_idx;

CREATE INDEX IF NOT EXISTS uk_countries_geom_idx
    ON gis_data.uk_countries USING gist
    (geom)
    TABLESPACE pg_default;