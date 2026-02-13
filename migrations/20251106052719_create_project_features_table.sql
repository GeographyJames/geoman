CREATE SEQUENCE app.project_features_id_seq;

CREATE TABLE app.project_features (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_id integer NOT NULL REFERENCES app.projects(id),
    collection_id integer NOT NULL REFERENCES app.collections(id),
    name text NOT NULL,
    is_primary boolean NOT NULL DEFAULT false,
    status app.status NOT NULL DEFAULT 'ACTIVE',
    properties JSONB NOT NULL DEFAULT '{}',
    added_by integer NOT NULL REFERENCES app.users(id),
    added timestamptz NOT NULL DEFAULT now(),
    last_updated_by integer NOT NULL REFERENCES app.users(id),
    last_updated timestamptz NOT NULL DEFAULT now(),
    geom geometry(GEOMETRY) NOT NULL CHECK (ST_IsValid(geom)),

    CHECK (NOT (is_primary AND status IN ('DELETED', 'ARCHIVED')))
);

CREATE UNIQUE INDEX idx_one_primary_per_project_collection
ON app.project_features(project_id, collection_id)
WHERE is_primary = true;

CREATE INDEX idx_features_project ON app.project_features(project_id);
CREATE INDEX idx_features_collection ON app.project_features(collection_id);
CREATE INDEX idx_features_status ON app.project_features(status);



