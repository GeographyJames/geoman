/// Row alias should match corresponding struct field name
pub fn user_row_fragment(user_alias: &str, row_alias: &str) -> String {
    let alias = user_alias;

    format!(
        "ROW({alias}.id, {alias}.first_name, {alias}.last_name, {alias}.clerk_id, \
 ROW(t_{alias}.id, t_{alias}.name)::app.team)::app.user AS {row_alias}"
    )
}

// Col should match user id column name to join on
pub fn user_join_fragment(user_alias: &str, col: &str) -> String {
    let alias = user_alias;
    format!(
        "JOIN app.users {alias} ON {alias}.id = p.{col} \
           {}",
        team_join_fragment(user_alias)
    )
}

pub fn team_join_fragment(user_alias: &str) -> String {
    let alias = user_alias;
    format!("JOIN app.teams t_{alias} ON {alias}.team_id = t_{alias}.id")
}
