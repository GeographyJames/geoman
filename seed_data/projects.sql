INSERT INTO app.projects (
    name,
    slug,
    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project',
    'test-project',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);
