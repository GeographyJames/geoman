INSERT INTO app.users (
    username, admin, team_id, clerk_id, first_name, last_name
) VALUES (
    'root', true, (SELECT id FROM app.teams WHERE name = 'root'), 'user_34TBak0wKXjYNSdz8EsCnCTrlVY', 'root', 'user'
);