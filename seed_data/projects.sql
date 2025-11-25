INSERT INTO app.projects (
    name,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

INSERT INTO app.projects (
    name,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project2',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

INSERT INTO app.projects (
    name,

    owner,
    added_by,
    last_updated_by
) VALUES (
    'Test Project3',

    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);
