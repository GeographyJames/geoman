use domain::{Team, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_teams_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let _team = app.generate_team_id(Some(&auth)).await;
    let response = app.teams_service.get(&app.api_client, Some(&auth)).await;
    assert_ok(&response);
    let teams: Vec<Team> = handle_json_response(response)
        .await
        .expect("failed to retrieve teams");
    assert_eq!(teams.len(), 1);
}
