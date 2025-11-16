CREATE TABLE app.feature_objects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_feature_id integer NOT NULL REFERENCES app.project_features(id),
    geom geometry(GEOMETRY) NOT NULL CHECK (ST_IsValid(geom)),
    properties JSONB
);

CREATE INDEX idx_feature_objects_geom ON app.feature_objects USING GIST(geom);
CREATE INDEX idx_feature_objects_feature ON app.feature_objects(project_feature_id);