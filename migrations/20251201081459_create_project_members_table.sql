CREATE TABLE app.project_members (
    project_id integer NOT NULL REFERENCES app.projects(id) ON DELETE CASCADE,
    user_id integer NOT NULL REFERENCES app.users(id) ON DELETE RESTRICT,
    PRIMARY KEY (project_id, user_id)
);

CREATE INDEX idx_project_members_project ON app.project_members(project_id);
CREATE INDEX idx_project_members_user ON app.project_members(user_id);