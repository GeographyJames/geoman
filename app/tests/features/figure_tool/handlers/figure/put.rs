use geoman::app::features::figure_tool::dtos::figure::FigureOutputDTO;

use crate::{app::TestApp, helpers::assert_ok};

#[tokio::test]
async fn put_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;

    let figure_id = app.generate_figure_id(project_id).await;
    let mut figure: FigureOutputDTO = app
        .figures_service
        .get_by_id(&app.api_client, &figure_id)
        .await
        .json()
        .await
        .expect("failed to deserialize json");

    let new_title = uuid::Uuid::new_v4().to_string();
    figure.properties.title = Some(new_title.clone());
    let response = app
        .figures_service
        .put_json(&app.api_client, &figure_id, &figure.clone())
        .await;
    assert_ok(&response);
    let updated_figure: FigureOutputDTO = app
        .figures_service
        .get_by_id(&app.api_client, figure_id)
        .await
        .json()
        .await
        .expect("failedo to deserialize json");
    assert_eq!(
        updated_figure
            .properties
            .title
            .expect("updated figure has no title"),
        new_title
    );
}
