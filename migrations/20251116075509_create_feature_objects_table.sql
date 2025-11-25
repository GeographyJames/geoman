CREATE TABLE app.feature_objects (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    collection_id integer NOT NULL,
    project_feature_id integer NOT NULL,
    geom geometry(GEOMETRY) NOT NULL CHECK (ST_IsValid(geom)),
    properties JSONB,
    FOREIGN KEY (collection_id, project_feature_id)
        REFERENCES app.project_features(collection_id, id)
);

CREATE INDEX idx_feature_objects_geom ON app.feature_objects USING GIST(geom);
CREATE INDEX idx_feature_objects_feature ON app.feature_objects(collection_id, project_feature_id);