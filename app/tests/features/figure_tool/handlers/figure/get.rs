use app::{
    features::figure_tool::{
        dtos::FigureOutputDTO,
        handlers::figure::{
            FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, PgTablePayload,
        },
        ids::FigureId,
        {FigureLayerDatasourceOutput, ProjectLayer},
    },
};

use crate::common::{TestApp, helpers::assert_ok};
use crate::features::figure_tool::handlers::project_layer::VALID_TABLE_NAMES;

#[tokio::test]
async fn get_figures_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let _figure_id = app.generate_figure_id(Some(&user), project_id).await;
    let response = app
        .figures_service
        .get_with_params(&app.api_client, Some(&user), &[("project", project_id.0)])
        .await;
    assert_ok(&response);
    let _figures: Vec<FigureOutputDTO> = response.json().await.expect("failed to deserialize json");
}

#[tokio::test]
async fn get_figures_works_with_missing_project_layer() {
    let (app, user, project_id) = TestApp::with_project().await;
    let layer = FigureLayerPayload {
        properties: Default::default(),
        style_id: None,
        source: FigureLayerDatasourcePayload::PgTable(PgTablePayload {
            table: "table".into(),
            schema: "shema".into(),
        }),
    };
    let mut figure_payload = FigurePayload::new(project_id);
    figure_payload.layers = Some(vec![layer]);
    let _figure_id: FigureId = app
        .figures_service
        .post_json(&app.api_client, Some(&user), &figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialize json");
    let response = app
        .figures_service
        .get_with_params(&app.api_client, Some(&user), &[("project", project_id.0)])
        .await;
    assert_ok(&response);
    let mut figures: Vec<FigureOutputDTO> =
        response.json().await.expect("failed to deserialize json");
    assert_eq!(figures.len(), 1);
    let mut layers = figures.pop().unwrap().layers.expect("no layers");
    assert_eq!(layers.len(), 1);
    if let FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Invalid(_)) =
        layers.pop().unwrap().source
    {
    } else {
        panic!("datasource should be invalid")
    }
}

#[tokio::test]
async fn get_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let boundary = app
        .generate_primary_boundary_id(project_id, Some(&user))
        .await;
    let mut figure_payload = FigurePayload::new(project_id);
    let layer = FigureLayerPayload {
        properties: Default::default(),
        source: FigureLayerDatasourcePayload::SiteBoundary(boundary.feature_id),
        style_id: None,
    };
    figure_payload.layers = Some(vec![layer]);
    let figure_id: FigureId = app
        .figures_service
        .post_json(&app.api_client, Some(&user), &figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialize json to figure id");
    let response = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await;
    assert_ok(&response);
    let figure: FigureOutputDTO = response.json().await.expect("failed to deserialize json");
    assert!(!figure.layers.expect("no layers").is_empty());
}

#[tokio::test]
pub async fn get_figure_with_project_layer_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let mut figure = FigurePayload::new(project_id);
    let layers: Vec<FigureLayerPayload> = VALID_TABLE_NAMES
        .iter()
        .map(|table| {
            FigureLayerPayload::new(FigureLayerDatasourcePayload::PgTable(PgTablePayload {
                table: (*table).into(),
                schema: "project_data".into(),
            }))
        })
        .collect();
    figure.layers = Some(layers);
    let response = app
        .figures_service
        .post_json(&app.api_client, Some(&user), &figure)
        .await;
    assert_ok(&response);
    let figure_id: FigureId = response.json().await.expect("failed to deserialize json");
    let response = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await;
    assert_ok(&response);
}
