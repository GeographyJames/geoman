INSERT INTO app.projects (
    name,
    country_code,
    slug,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project',
    'GB',
    'test-project',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

INSERT INTO app.projects (
    name,
    country_code,
    slug,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project2',
    'GB',
    'test-project2',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

INSERT INTO app.projects (
    name,
    country_code,
    slug,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project3',
    'GB',
    'test-project3',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);


INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project'), (SELECT id FROM app.technologies WHERE name = 'wind')
);

INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project2'), (SELECT id FROM app.technologies WHERE name = 'wind')),
    ((SELECT id FROM app.projects WHERE name = 'Test Project2'), (SELECT id FROM app.technologies WHERE name = 'solar')
);

INSERT INTO app.project_technologies (project_id, technology_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project3'), (SELECT id FROM app.technologies WHERE name = 'battery')
);


INSERT INTO app.project_subdivisions (project_id, subdivision_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project'), (SELECT id FROM app.subdivisions WHERE name = 'Scotland')
);

INSERT INTO app.project_subdivisions (project_id, subdivision_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project2'), (SELECT id FROM app.subdivisions WHERE name = 'England')
);

INSERT INTO app.project_subdivisions (project_id, subdivision_id) VALUES (
    (SELECT id FROM app.projects WHERE name = 'Test Project3'), (SELECT id FROM app.subdivisions WHERE name = 'Wales')
);