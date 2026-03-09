CREATE TABLE app.base_map_data_providers (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    copyright_text TEXT
)