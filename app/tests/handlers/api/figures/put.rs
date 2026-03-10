use app::handlers::api::figures::FigurePayload;

use crate::common::{TestApp, helpers::assert_status};

#[tokio::test]
async fn patch_figure_works() {
    let (app, auth, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&auth), project_id).await;
    let mut payload = FigurePayload::new(project_id);
    payload.scale = Some(25000);
    let response = app
        .figures_service
        .patch_json(&app.api_client, figure_id.0, Some(&auth), &payload)
        .await;
    assert_status(&response, 204);
}
