use app::features::figure_tool::{handlers::figure::FigurePayload, ids::FigureId};

use crate::common::{TestApp, helpers::assert_ok};

#[tokio::test]
async fn post_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let payload = FigurePayload::new(project_id);
    let response = app
        .figures_service
        .post_json(&app.api_client, Some(&user), &payload)
        .await;
    assert_ok(&response);
    let _figure_id: FigureId = response
        .json()
        .await
        .expect("failed to deserialise figure id");
}
