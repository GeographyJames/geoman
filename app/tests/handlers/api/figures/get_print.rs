use app::URLS;
use domain::{FigureId, figure::FigureOutputDTO};

use app::handlers::api::figures::{
    FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, PgTablePayload,
};

use crate::{
    common::{
        TestApp,
        helpers::{
            assert_is_qgis_project, assert_ok, assert_response_is_jpg, assert_response_is_pdf,
            auth_request,
        },
    },
    handlers::api::project_layers::VALID_TABLE_NAMES,
};

#[tokio::test]
async fn get_figure_pdf_works() {
    let (app, auth, project_id) = TestApp::with_project().await;

    let boundary_id = app
        .generate_primary_boundary_id(project_id, Some(&auth))
        .await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&auth))
        .await;

    let mut layers = vec![
        FigureLayerPayload::new(FigureLayerDatasourcePayload::SiteBoundary(
            boundary_id.feature_id,
        )),
        FigureLayerPayload::new(FigureLayerDatasourcePayload::TurbineLayout(layout_id)),
    ];
    for table in VALID_TABLE_NAMES {
        layers.push(FigureLayerPayload::new(
            FigureLayerDatasourcePayload::PgTable(PgTablePayload {
                table: table.into(),
                schema: "project_data".into(),
            }),
        ));
    }
    let mut figure_payload = FigurePayload::new(project_id);
    figure_payload.layers = Some(layers);

    let figure_id: FigureId = app
        .post_figure(Some(&auth), &figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialize figure id");

    let response = auth_request(
        app.api_client.get(&format!(
            "{}{}/{}/pdf",
            URLS.api.base, URLS.api.figures, figure_id.0
        )),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);
    assert_response_is_pdf(response).await;

    let figure: FigureOutputDTO = app
        .figures_service
        .get_one(&app.api_client, Some(&auth), figure_id.0)
        .await
        .json()
        .await
        .expect("failed to deserialize figure");

    // Check qgis project was saved
    let qgis_project_name = figure.qgis_project_uuid.to_string();
    let qgis_project = app
        .qgis_projects_service
        .get_one(&app.api_client, Some(&auth), &qgis_project_name)
        .await;
    assert_is_qgis_project(qgis_project).await;

    // Update the figure
    app.figures_service
        .patch_json(
            &app.api_client,
            figure_id.0,
            Some(&auth),
            &FigurePayload::new(project_id),
        )
        .await;

    // Generate a new PDF
    let _response = auth_request(
        app.api_client.get(&format!(
            "{}{}/{}/pdf",
            URLS.api.base, URLS.api.figures, figure_id.0
        )),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");

    // Assert the old qgis project still exists (PDF keeps old projects)
    let qgis_project = app
        .qgis_projects_service
        .get_one(&app.api_client, Some(&auth), &qgis_project_name)
        .await;
    assert_is_qgis_project(qgis_project).await;
}

#[tokio::test]
async fn get_figure_jpg_works() {
    let (app, auth, project_id) = TestApp::with_project().await;

    let figure_id = app.generate_figure_id(Some(&auth), project_id).await;

    let response = auth_request(
        app.api_client.get(&format!(
            "{}{}/{}/jpg",
            URLS.api.base, URLS.api.figures, figure_id.0
        )),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);
    assert_response_is_jpg(response).await;

    let figure: FigureOutputDTO = app
        .figures_service
        .get_one(&app.api_client, Some(&auth), figure_id.0)
        .await
        .json()
        .await
        .expect("failed to deserialize figure");

    // Check qgis project was saved
    let qgis_project_name = format!("{}_low-res", figure.qgis_project_uuid);
    let qgis_project = app
        .qgis_projects_service
        .get_one(&app.api_client, Some(&auth), &qgis_project_name)
        .await;
    assert_is_qgis_project(qgis_project).await;

    // Update the figure
    app.figures_service
        .patch_json(
            &app.api_client,
            figure_id.0,
            Some(&auth),
            &FigurePayload::new(project_id),
        )
        .await;

    // Generate a new JPG
    let _response = auth_request(
        app.api_client.get(&format!(
            "{}{}/{}/jpg",
            URLS.api.base, URLS.api.figures, figure_id.0
        )),
        Some(&auth),
    )
    .send()
    .await
    .expect("failed to execute request");

    // Assert the old qgis project no longer exists (JPG regeneration replaces the stored project)
    let qgis_project = app
        .qgis_projects_service
        .get_one(&app.api_client, Some(&auth), &qgis_project_name)
        .await;
    assert_eq!(qgis_project.status().as_u16(), 404);
}
