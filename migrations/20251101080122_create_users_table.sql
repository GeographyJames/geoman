CREATE TABLE app.users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text NOT NULL UNIQUE DEFAULT gen_random_uuid()::text,
    first_name text NOT NULL,
    last_name text NOT NULL,
    clerk_id text UNIQUE,
    admin bool NOT NULL DEFAULT FALSE,
    team_id integer REFERENCES app.teams(id),
    added timestamptz NOT NULL DEFAULT now()
);

INSERT INTO app.users (
    id, username, admin, team_id, first_name, last_name
) OVERRIDING SYSTEM VALUE VALUES (
    0, 'root', true, 0, 'root', 'user'
);