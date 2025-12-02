CREATE TABLE app.technologies (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO app.technologies (
    name
    )
    VALUES ('wind'), ('solar'), ('battery'), ('hydrogen');