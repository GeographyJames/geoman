CREATE TABLE app.project_technologies (
    project_id INTEGER NOT NULL REFERENCES app.projects(id) ON DELETE CASCADE,
    technology_id INTEGER NOT NULL REFERENCES app.technologies ON DELETE RESTRICT,
    PRIMARY KEY (project_id, technology_id),
    added TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );

  CREATE INDEX idx_project_technologies_project ON app.project_technologies(project_id);
  CREATE INDEX idx_project_technologies_technology ON app.project_technologies(technology_id);

