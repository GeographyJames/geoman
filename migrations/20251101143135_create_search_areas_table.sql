
CREATE TABLE app.search_areas
(
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    team_id integer NOT NULL REFERENCES app.teams(id),
    code text,
    slug text NOT NULL UNIQUE CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    status app.status NOT NULL DEFAULT 'ACTIVE',
    added timestamptz DEFAULT now(),
    added_by integer NOT NULL REFERENCES app.users(id),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    geom geometry(MultiPolygon,4326) NOT NULL CHECK (st_isvalid(geom))
);



CREATE INDEX idx_search_areas_geom ON app.search_areas USING GIST(geom);

  CREATE INDEX idx_search_areas_team_id ON app.search_areas(team_id);
  CREATE INDEX idx_search_areas_slug ON app.search_areas(slug);