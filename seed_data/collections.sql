INSERT INTO app.collections (
    title,

    geometry_type,
    added_by,
    last_updated_by
) VALUES (
    'site boundaries',

    'MULTIPOLYGON',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    'access tracks',

    'MULTILINESTRING',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    'cable routes',

    'MULTILINESTRING',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);