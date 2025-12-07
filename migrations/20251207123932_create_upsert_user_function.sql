-- Create function to upsert user and return user context
CREATE OR REPLACE FUNCTION app.upsert_user_and_get_context(p_clerk_id TEXT)
RETURNS TABLE(user_id INTEGER, user_team_id INTEGER) AS $$
BEGIN
  -- Insert user with defaults if doesn't exist
  INSERT INTO app.users (clerk_id, first_name, last_name, team_id)
  VALUES (p_clerk_id, 'New', 'User', -1)
  ON CONFLICT (clerk_id) DO NOTHING;

  -- Return user id and team_id
  RETURN QUERY
  SELECT id, team_id
  FROM app.users
  WHERE clerk_id = p_clerk_id;
END;
$$ LANGUAGE plpgsql;
