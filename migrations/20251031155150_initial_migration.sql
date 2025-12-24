CREATE SCHEMA app;
CREATE SCHEMA gis_data;
CREATE TYPE app.status AS ENUM ('ACTIVE', 'ARCHIVED', 'DELETED');
CREATE TYPE app.visibility AS ENUM ('PRIVATE', 'TEAM', 'PUBLIC');
CREATE TYPE geometry_type AS ENUM (
    'POINT',
    'LINESTRING',
    'POLYGON',
    'MULTIPOINT',
    'MULTILINESTRING',
    'MULTIPOLYGON',
    'GEOMETRYCOLLECTION'
);

CREATE TYPE app.team AS (
    id INTEGER,
    name TEXT
);

CREATE TYPE app.user AS (
    id INTEGER,
    first_name TEXT,
    last_name TEXT,
    clerk_id TEXT,
    team app.team
    );

CREATE TYPE app.subdivision AS (
    id INTEGER,
    country_code TEXT,
    subdivision_code TEXT,
    name TEXT
);