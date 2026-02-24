INSERT INTO app.teams (name, business_unit_id, added_by, last_updated_by) VALUES
    ('Backend',        (SELECT id FROM app.business_units WHERE name = 'Engineering'), 0, 0),
    ('Frontend',       (SELECT id FROM app.business_units WHERE name = 'Engineering'), 0, 0),
    ('Data',           (SELECT id FROM app.business_units WHERE name = 'Operations'), 0, 0),
    ('Infrastructure', (SELECT id FROM app.business_units WHERE name = 'Operations'), 0, 0);
