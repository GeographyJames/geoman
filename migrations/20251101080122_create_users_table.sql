CREATE TABLE app.users (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    clerk_id text,
    admin bool NOT NULL DEFAULT FALSE,
    team_id integer REFERENCES app.teams(id),
    added timestamptz NOT NULL DEFAULT now()
)