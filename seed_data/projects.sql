INSERT INTO app.projects (
    name,
    country_code,
    slug,
    subdivisions,
    team_id,
    

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project',
    'GB',
    'test-project',
    ARRAY['GB-ENG'], 0, 0, 0, 0
),

 (
    'Test Project2',
    'GB',
    'test-project2',ARRAY['GB-ENG'], 0, 0, 0, 0
),

 (
    'Test Project3',
    'GB',
    'test-project3',ARRAY['GB-ENG'], 0, 0, 0, 0
);


INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project'), (SELECT id FROM app.technologies WHERE name = 'onshore wind')
);

INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project2'), (SELECT id FROM app.technologies WHERE name = 'onshore wind')),
    ((SELECT id FROM app.projects WHERE name = 'Test Project2'), (SELECT id FROM app.technologies WHERE name = 'solar')
);

INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project3'), (SELECT id FROM app.technologies WHERE name = 'battery')
);


