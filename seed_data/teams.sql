INSERT INTO app.teams (name, business_unit_id) VALUES
    ('Backend',        (SELECT id FROM app.business_units WHERE name = 'Engineering')),
    ('Frontend',       (SELECT id FROM app.business_units WHERE name = 'Engineering')),
    ('Data',           (SELECT id FROM app.business_units WHERE name = 'Operations')),
    ('Infrastructure', (SELECT id FROM app.business_units WHERE name = 'Operations'));
