use domain::Team;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_teams_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let response = app.teams_service.get(&app.api_client, Some(&auth)).await;
    assert_ok(&response);
    let teams: Vec<Team> = handle_json_response(response)
        .await
        .expect("failed to retrieve teams");
    assert_eq!(teams.len(), 2);
}
