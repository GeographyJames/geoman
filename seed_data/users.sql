INSERT INTO app.users (
    username, admin, team_id
) VALUES (
    'root', true, (SELECT id FROM app.teams WHERE name = 'root')
);