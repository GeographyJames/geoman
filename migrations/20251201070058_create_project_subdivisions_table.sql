CREATE TABLE app.project_subdivisions (
    project_id integer NOT NULL
        REFERENCES app.projects(id)
        ON DELETE CASCADE,

    subdivision_id integer NOT NULL
        REFERENCES app.subdivisions(id)
        ON DELETE RESTRICT,

    PRIMARY KEY (project_id, subdivision_id)
);


CREATE INDEX idx_project_subdivisions_project
    ON app.project_subdivisions(project_id);

CREATE INDEX idx_project_subdivisions_subdivision
    ON app.project_subdivisions(subdivision_id);
