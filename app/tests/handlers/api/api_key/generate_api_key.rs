use crate::common::{
    TestApp,
    helpers::{assert_ok, handle_json_response},
};

#[actix_web::test]
async fn generate_api_key_works() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let _user_id = app.insert_user(team_id, Some(&app.auth.test_user_id)).await;

    let token = app
        .auth
        .get_test_session_token(&app.api_client.client)
        .await;
    let response = app
        .api_keys_service
        .generate_api_key(&app.api_client, Some(&token))
        .await;
    assert_ok(&response);
    let _key: serde_json::Map<_, serde_json::Value> = handle_json_response(response)
        .await
        .expect("failed to retrive key");
}
