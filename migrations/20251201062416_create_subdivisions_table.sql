CREATE TABLE app.subdivisions (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    country_code character(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    subdivision_code varchar(3) NOT NULL CHECK (subdivision_code ~ '^[A-Z0-9]{1,3}$'),
    name text NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (country_code, subdivision_code)
);

CREATE INDEX idx_subdivisions_country_code ON app.subdivisions(country_code);