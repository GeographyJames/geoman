use crate::common::{AppBuilder, helpers::assert_ok};

#[tokio::test]
async fn post_project_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let response = app
        .projects_service
        .post_json(&app.api_client, Some(&token), &())
        .await;
    assert_ok(&response)
}
