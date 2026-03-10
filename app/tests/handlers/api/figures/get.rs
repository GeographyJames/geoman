use app::{
    URLS,
    handlers::api::figures::{FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload},
};
use domain::FigureId;

use crate::common::{
    TestApp,
    helpers::{assert_ok, auth_request},
};

#[tokio::test]
async fn get_figures_works() {
    let (app, auth, project_id) = TestApp::with_project().await;
    let _figure_id = app.generate_figure_id(Some(&auth), project_id).await;
    let response = auth_request(
        app.api_client
            .get(&format!("{}{}", URLS.api.base, URLS.api.figures))
            .query(&[("project_id", project_id.0)]),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);
    let _figures: Vec<serde_json::Value> =
        response.json().await.expect("failed to deserialize json");
}

#[tokio::test]
async fn get_figure_works() {
    let (app, auth, project_id) = TestApp::with_project().await;
    let boundary_id = app
        .generate_primary_boundary_id(project_id, Some(&auth))
        .await;
    let layer = FigureLayerPayload {
        properties: Default::default(),
        source: FigureLayerDatasourcePayload::SiteBoundary(boundary_id.feature_id),
        style_id: None,
    };
    let mut figure_payload = FigurePayload::new(project_id);
    figure_payload.layers = Some(vec![layer]);
    let figure_id: FigureId = app
        .post_figure(Some(&auth), &figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialize figure id");
    let response = app
        .figures_service
        .get_one(&app.api_client, Some(&auth), figure_id.0)
        .await;
    assert_ok(&response);
    let figure: serde_json::Value = response.json().await.expect("failed to deserialize json");
    assert!(
        figure["layers"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        "expected layers to be non-empty"
    );
}

// The following tests require additional infrastructure not yet ported:
// - FigureFormat enum (from get_print handler)
// - qgis_projects_service on TestApp
// - assert_is_qgis_project, assert_response_is_pdf, assert_response_is_jpg helpers
// - PgTable datasource variant (commented out in payload)

// #[tokio::test]
// async fn get_figures_works_with_missing_project_layer() { ... }

// #[tokio::test]
// async fn get_figure_pdf_works() { ... }

// #[tokio::test]
// async fn get_figure_jpg_works() { ... }

// #[tokio::test]
// pub async fn get_figure_qgis_project_works() { ... }

// #[tokio::test]
// pub async fn get_figure_with_project_layer_works() { ... }
