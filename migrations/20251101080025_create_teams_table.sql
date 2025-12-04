CREATE TABLE app.teams (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    added timestamptz NOT NULL DEFAULT now()
);

INSERT INTO app.teams (
    id, name
    ) OVERRIDING SYSTEM VALUE VALUES (0, 'root');