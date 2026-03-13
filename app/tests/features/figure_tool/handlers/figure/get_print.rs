use app::{
    URLS,
    features::figure_tool::{
        FigureFormat, PrintResolution,
        dtos::FigureOutputDTO,
        handlers::figure::{
            FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, FigureUpdatePayload,
            PgTablePayload,
        },
        ids::FigureId,
    },
};

use crate::common::{
    TestApp,
    helpers::{assert_ok, auth_request},
};
use crate::features::figure_tool::handlers::project_layer::VALID_TABLE_NAMES;

pub async fn assert_response_is_pdf(response: reqwest::Response) {
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok());

    assert_eq!(content_type, Some("application/pdf"));

    let body = response.bytes().await.expect("failed to get response body");

    assert!(!body.is_empty(), "PDF response body should not be empty");
    assert!(
        body.starts_with(b"%PDF-"),
        "Response should start with PDF magic bytes"
    );
}

pub async fn assert_response_is_jpg(response: reqwest::Response) {
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok());

    assert_eq!(content_type, Some("image/jpeg"));

    let body = response.bytes().await.expect("failed to get response body");

    assert!(!body.is_empty(), "JPG response body should not be empty");
    assert!(
        body.starts_with(b"\xff\xd8\xff"),
        "Response should start with JPG magic bytes"
    );
}

pub async fn assert_is_qgis_project(response: reqwest::Response) {
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .expect("response has no content-type header"),
        "application/octet-stream"
    );

    let body = response
        .bytes()
        .await
        .expect("Failed to read response body");

    // Check ZIP magic bytes (first 4 bytes should be "PK\x03\x04")
    assert!(body.len() >= 4, "Response body too short to be a ZIP file");
    assert_eq!(&body[0..4], b"PK\x03\x04", "Not a valid ZIP file");
}

#[tokio::test]
async fn get_figure_jpg_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let figure_id = app.generate_figure_id(Some(&user), project_id).await;
    let response = auth_request(
        app.api_client.get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::jpg
        )),
        Some(&user),
    )
    .send()
    .await
    .expect("failed to execute request");
    assert_ok(&response);
    assert_response_is_jpg(response).await;
    // check the qgis project is in the database
    let figure: FigureOutputDTO = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await
        .json()
        .await
        .expect("failed to deseriailze figure");

    let qgis_project = app
        .qgis_projects_service
        .get_one(
            &app.api_client,
            Some(&user),
            figure.qgis_project_name(&PrintResolution::Low).0,
        )
        .await;
    assert_is_qgis_project(qgis_project).await;

    // update the project
    app.figures_service
        .patch_json(
            &app.api_client,
            figure_id,
            Some(&user),
            &FigureUpdatePayload::default(),
        )
        .await;

    // generate new jpg
    let _response = auth_request(
        app.api_client.get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::jpg
        )),
        Some(&user),
    )
    .send()
    .await
    .expect("failed to execute request");

    let qgis_project = app
        .qgis_projects_service
        .get_one(
            &app.api_client,
            Some(&user),
            figure.qgis_project_name(&PrintResolution::Low).0,
        )
        .await;

    // assert old qgis project does not exist
    assert_eq!(
        qgis_project.status().as_u16(),
        404,
        "qgis project not deleted"
    )
}

#[tokio::test]
async fn get_figure_pdf_works() {
    let (app, user, project_id) = TestApp::with_project().await;

    let boundary = app
        .generate_primary_boundary_id(project_id, Some(&user))
        .await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;
    let mut figure = FigurePayload::new(project_id);

    let mut layers = Vec::new();
    layers.push(FigureLayerPayload::new(
        FigureLayerDatasourcePayload::SiteBoundary(boundary.feature_id),
    ));
    layers.push(FigureLayerPayload::new(
        FigureLayerDatasourcePayload::TurbineLayout(layout_id),
    ));
    for table in VALID_TABLE_NAMES {
        let ds = FigureLayerDatasourcePayload::PgTable(PgTablePayload {
            table: table.into(),
            schema: "project_data".into(),
        });
        layers.push(FigureLayerPayload::new(ds));
    }
    figure.layers = Some(layers);
    let response = app
        .figures_service
        .post_json(&app.api_client, Some(&user), &figure)
        .await;
    assert_ok(&response);
    let figure_id: FigureId = response.json().await.expect("failed to deserialize json");

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
    assert_response_is_pdf(response).await;
    let figure: FigureOutputDTO = app
        .figures_service
        .get_one(&app.api_client, Some(&user), figure_id)
        .await
        .json()
        .await
        .expect("failed to deseriailze figure");
    // Check qgis project is saved
    let qgis_project = app
        .qgis_projects_service
        .get_one(
            &app.api_client,
            Some(&user),
            figure.qgis_project_name(&PrintResolution::High).0,
        )
        .await;
    assert_is_qgis_project(qgis_project).await;

    // update the project
    app.figures_service
        .patch_json(
            &app.api_client,
            figure_id,
            Some(&user),
            &FigureUpdatePayload::default(),
        )
        .await;

    // generate new pdf
    let _response = auth_request(
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
    let qgis_project = app
        .qgis_projects_service
        .get_one(
            &app.api_client,
            Some(&user),
            figure.qgis_project_name(&PrintResolution::High).0,
        )
        .await;

    // assert old qgis project does exist
    assert_is_qgis_project(qgis_project).await
}
