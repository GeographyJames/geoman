use app::URLS;

use crate::common::{
    TestApp,
    helpers::{assert_ok, auth_request},
};
use crate::features::figure_tool::handlers::figure::get_print::assert_is_qgis_project;

#[tokio::test]
async fn get_figure_qgis_project_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&user), project_id).await;
    let response = auth_request(
        app.api_client.get(format!(
            "{}{}/{}/qgz",
            URLS.api.base, URLS.api.figures, figure_id,
        )),
        Some(&user),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);
    assert_is_qgis_project(response).await;
}
