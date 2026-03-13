use app::{
    URLS,
    features::figure_tool::{FigureFormat, PrintResolution, dtos::FigureOutputDTO},
};

use crate::common::{
    TestApp,
    helpers::{assert_ok, auth_request},
};
use crate::features::figure_tool::handlers::figure::get_print::assert_is_qgis_project;

#[tokio::test]
async fn get_qgis_project_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&user), project_id).await;

    // generate the qgis project via the print endpoint
    let response = auth_request(
        app.api_client.get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::pdf
        )),
        Some(&user),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);

    let figure: FigureOutputDTO = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await
        .json()
        .await
        .expect("failed to deserialize figure");

    let response = app
        .qgis_projects_service
        .get_one(
            &app.api_client,
            Some(&user),
            figure.qgis_project_name(&PrintResolution::High).0,
        )
        .await;
    assert_is_qgis_project(response).await;
}
