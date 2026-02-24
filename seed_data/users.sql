INSERT INTO app.users (username, first_name, last_name, team_id) VALUES
    ('alice.jones',   'Alice',   'Jones',   (SELECT id FROM app.teams WHERE name = 'Backend')),
    ('bob.smith',     'Bob',     'Smith',   (SELECT id FROM app.teams WHERE name = 'Backend')),
    ('carol.white',   'Carol',   'White',   (SELECT id FROM app.teams WHERE name = 'Frontend')),
    ('dan.brown',     'Dan',     'Brown',   (SELECT id FROM app.teams WHERE name = 'Frontend')),
    ('eve.taylor',    'Eve',     'Taylor',  (SELECT id FROM app.teams WHERE name = 'Data')),
    ('frank.miller',  'Frank',   'Miller',  (SELECT id FROM app.teams WHERE name = 'Data')),
    ('grace.wilson',  'Grace',   'Wilson',  (SELECT id FROM app.teams WHERE name = 'Infrastructure')),
    ('harry.moore',   'Harry',   'Moore',   (SELECT id FROM app.teams WHERE name = 'Infrastructure'));
