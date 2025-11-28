INSERT INTO app.users (
    username, admin, team_id, clerk_id
) VALUES (
    'root', true, (SELECT id FROM app.teams WHERE name = 'root'), 'user_34TBak0wKXjYNSdz8EsCnCTrlVY'
);