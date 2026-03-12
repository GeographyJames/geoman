use app::features::figure_tool::{
    dtos::{FigureOutputDTO, FigureProperties},
    handlers::figure::FigureUpdatePayload,
};
use domain::enums::Status;

use crate::common::{
    TestApp,
    helpers::{assert_ok, assert_status},
};

#[tokio::test]
async fn delete_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&user), project_id).await;

    let payload = FigureUpdatePayload {
        status: Some(Status::Deleted),
        ..Default::default()
    };

    let response = app
        .figures_service
        .patch_json(&app.api_client, figure_id, Some(&user), &payload)
        .await;
    assert_status(&response, 204);

    let response = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await;
    assert_status(&response, 404);

    let response = app
        .figures_service
        .get_with_params(&app.api_client, Some(&user), &[("project", project_id.0)])
        .await;
    assert_ok(&response);
    let figures: Vec<FigureOutputDTO> = response.json().await.expect("failed to deserialize json");
    assert!(figures.is_empty());
}

#[tokio::test]
async fn patch_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&user), project_id).await;

    let new_title = uuid::Uuid::new_v4().to_string();
    let payload = FigureUpdatePayload {
        properties: Some(FigureProperties {
            title: Some(new_title.clone()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response = app
        .figures_service
        .patch_json(&app.api_client, figure_id, Some(&user), &payload)
        .await;
    assert_status(&response, 204);

    let response = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await;
    assert_ok(&response);
    let updated: FigureOutputDTO = response.json().await.expect("failed to deserialize json");
    assert_eq!(
        updated.properties.title.expect("figure has no title"),
        new_title
    );
}
