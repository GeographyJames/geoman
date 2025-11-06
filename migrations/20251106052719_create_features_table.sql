CREATE TABLE app.features (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_id integer NOT NULL REFERENCES app.projects(id),
    collection_id integer NOT NULL REFERENCES app.collections(id),
    name text NOT NULL,
    is_primary boolean NOT NULL DEFAULT false,
    status app.status NOT NULL DEFAULT 'active',
    properties JSONB NOT NULL DEFAULT '{}',
    geom geometry(GEOMETRY) NOT NULL CHECK (ST_IsValid(geom)),
    
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    
    CHECK (NOT (is_primary AND status IN ('deleted', 'archived')))
);

CREATE UNIQUE INDEX idx_one_primary_per_project_collection
ON app.features(project_id, collection_id)
WHERE is_primary = true;
CREATE INDEX idx_features_geom ON app.features USING GIST(geom);
CREATE INDEX idx_features_project ON app.features(project_id);
CREATE INDEX idx_features_collection ON app.features(collection_id);
CREATE INDEX idx_features_status ON app.features(status);

CREATE TRIGGER update_last_updated_trigger
    BEFORE UPDATE ON app.features
    FOR EACH ROW
    EXECUTE FUNCTION update_last_updated();