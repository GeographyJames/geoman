CREATE TABLE app.business_units (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    added timestamptz NOT NULL DEFAULT now(),
    last_updated timestamptz DEFAULT now()

);

CREATE TABLE app.teams (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    business_unit_id integer REFERENCES app.business_units(id),    
    name text NOT NULL UNIQUE,
    added timestamptz NOT NULL DEFAULT now(),
    last_updated timestamptz DEFAULT now()


);

INSERT INTO app.teams (
    id, name
    ) OVERRIDING SYSTEM VALUE VALUES (0, 'root'), (-1, 'unassigned users');