CREATE TABLE app.users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text NOT NULL UNIQUE DEFAULT gen_random_uuid()::text,
    first_name text NOT NULL,
    last_name text NOT NULL,
    clerk_id text UNIQUE,
    admin bool NOT NULL DEFAULT FALSE,
    team_id integer NOT NULL REFERENCES app.teams(id),
    added timestamptz NOT NULL DEFAULT now()
);

INSERT INTO app.users (
    username, admin, team_id, first_name, last_name
) VALUES (
    'root', true, (SELECT id FROM app.teams WHERE name = 'root'), 'root', 'user'
);