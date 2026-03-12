use geoman::{
    app::{URLS, features::figure_tool::handlers::figure::FigureFormat},
    app::features::figure_tool::dtos::figure::FigureOutputDTO,
};

use crate::{
    app::TestApp,
    helpers::{assert_is_qgis_project, assert_ok},
};

#[tokio::test]
async fn get_qgis_project_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let figure_id = app.generate_figure_id(project_id).await;
    let figure: FigureOutputDTO = app
        .figures_service
        .get_by_id(&app.api_client, &figure_id)
        .await
        .json()
        .await
        .expect("failed to deserialize json");
    let response = app
        .api_client
        .get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::pdf
        ))
        .send()
        .await
        .expect("failed to execute request");
    assert_ok(&response);
    let response = app
        .qgis_projects_service
        .get_by_string_id(&app.api_client, figure.qgis_project_uuid.to_string())
        .await;
    assert_ok(&response);
    // Check content type
    assert_is_qgis_project(response).await;
}
