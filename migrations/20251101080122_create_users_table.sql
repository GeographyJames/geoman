CREATE TABLE app.users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text NOT NULL UNIQUE DEFAULT gen_random_uuid()::text,
    clerk_id text UNIQUE,
    admin bool NOT NULL DEFAULT FALSE,
    team_id integer NOT NULL REFERENCES app.teams(id),
    added timestamptz NOT NULL DEFAULT now()
);

