#[tokio::test]
pub async fn get_figure_qgis_project_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let figure_id = app.generate_figure_id(project_id).await;
    let response = app
        .api_client
        .get(format!(
            "{}{}/{}/qgz",
            URLS.api.base, URLS.api.figures, figure_id,
        ))
        .send()
        .await
        .expect("failed to execute request");
    assert_ok(&response);
    assert_is_qgis_project(response).await;
}
