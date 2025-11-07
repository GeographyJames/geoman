INSERT INTO app.collections (
    title,
    slug,
    geometry_type,
    added_by,
    last_updated_by
) VALUES (
    'site boundaries',
    'site-boundaries',
    'MULTIPOLYGON',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    'access tracks',
    'access-tracks',
    'MULTILINESTRING',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    'cable routes',
    'cable-routes',
    'MULTILINESTRING',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);