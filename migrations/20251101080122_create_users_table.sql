CREATE TABLE app.users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text,
    first_name text NOT NULL,
    last_name text NOT NULL,
    clerk_id text UNIQUE,
    admin bool NOT NULL DEFAULT FALSE,
    team_id integer NOT NULL REFERENCES app.teams(id),
    added timestamptz NOT NULL DEFAULT now()
);

INSERT INTO app.users (
    id, username, admin, team_id, first_name, last_name
) OVERRIDING SYSTEM VALUE VALUES (
    0, 'root-user', true, 0, 'root', 'user'
);