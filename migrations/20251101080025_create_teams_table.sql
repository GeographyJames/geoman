CREATE TABLE app.teams (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    added timestamptz NOT NULL DEFAULT now()
)