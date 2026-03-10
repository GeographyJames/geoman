use app::URLS;

use crate::common::{TestApp, helpers::{assert_ok, assert_status, auth_request}};

#[tokio::test]
async fn delete_figure_works() {
    let (app, auth, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&auth), project_id).await;

    let response = app
        .figures_service
        .delete(&app.api_client, figure_id.0, Some(&auth))
        .await;
    assert_status(&response, 204);

    let list_response = auth_request(
        app.api_client
            .get(&format!("{}{}", URLS.api.base, URLS.api.figures))
            .query(&[("project_id", project_id.0)]),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&list_response);
    let figures: Vec<serde_json::Value> = list_response.json().await.expect("failed to deserialize");
    assert!(figures.is_empty());
}
