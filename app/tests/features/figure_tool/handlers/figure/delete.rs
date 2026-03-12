use geoman::app::features::figure_tool::dtos::figure::FigureOutputDTO;

use crate::{app::TestApp, helpers::assert_ok};

#[tokio::test]
async fn delete_figure_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let figure_id = app.generate_figure_id(project_id).await;
    let figures: Vec<FigureOutputDTO> = app
        .figures_service
        .get_all_for_project(&app.api_client, project_id)
        .await
        .json()
        .await
        .expect("failed to deserialize json");
    assert_eq!(figures.len(), 1);
    let response = app
        .figures_service
        .delete(&app.api_client, &figure_id)
        .await;
    assert_ok(&response);
    let figures: Vec<FigureOutputDTO> = app
        .figures_service
        .get_all_for_project(&app.api_client, project_id)
        .await
        .json()
        .await
        .expect("failed to deserialzie json");
    assert!(figures.is_empty())
}
