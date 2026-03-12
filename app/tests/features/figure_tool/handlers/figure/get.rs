use geoman::app::{
    URLS,
    features::figure_tool::{
        dtos::figure::FigureOutputDTO,
        enums::{FigureLayerDatasourceOutput, ProjectLayer},
        handlers::figure::{
            FigureFormat, FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload,
            PgTablePayload,
        },
        ids::{FigureId, ProjectId, SiteBoundaryId, TurbineLayoutId},
        qgis_builder::PrintResolution,
    },
};

use crate::{
    app::TestApp,
    features::figure_tool::handlers::project_layer::VALID_TABLE_NAMES,
    helpers::{assert_is_qgis_project, assert_ok, assert_response_is_jpg, assert_response_is_pdf},
};

#[tokio::test]
async fn get_figures_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let _figure_id = app.generate_figure_id(project_id).await;
    let response = app
        .figures_service
        .get_all_for_project(&app.api_client, &project_id)
        .await;
    assert_ok(&response);
    let _figures: Vec<FigureOutputDTO> = response.json().await.expect("failed to deserialize json");
}

#[tokio::test]
async fn get_figures_works_with_missing_project_layer() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let layer = FigureLayerPayload {
        properties: Default::default(),
        style_id: None,
        source: FigureLayerDatasourcePayload::PgTable(PgTablePayload {
            table: "table".into(),
            schema: "shema".into(),
        }),
    };
    let mut figure_payload = FigurePayload::new(ProjectId(project_id.0));
    figure_payload.layers = Some(vec![layer]);
    let _figure_id: FigureId = app
        .post_figure(&figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialzie json");
    let response = app
        .figures_service
        .get_all_for_project(&app.api_client, &project_id)
        .await;
    assert_ok(&response);
    let mut figures: Vec<FigureOutputDTO> =
        response.json().await.expect("failed to deserialzie json");
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
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let boundary_id = app.generate_primary_boundary_id(&project_id).await;
    let mut figure_payload = FigurePayload::new(ProjectId(project_id.0));
    let layer = FigureLayerPayload {
        properties: Default::default(),
        source: FigureLayerDatasourcePayload::SiteBoundary(SiteBoundaryId(boundary_id.0)),
        style_id: None,
    };
    figure_payload.layers = Some(vec![layer]);
    let figure_id: FigureId = app
        .post_figure(&figure_payload)
        .await
        .json()
        .await
        .expect("failed to deserialize json to figure id");
    let response = app
        .figures_service
        .get_by_id(&app.api_client, &figure_id)
        .await;
    assert_ok(&response);
    let figure: FigureOutputDTO = response.json().await.expect("failed to deserialize json");
    assert!(!figure.layers.expect("no layers").is_empty());
}

#[tokio::test]
async fn get_figure_pdf_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;

    let boundary_id = app.generate_primary_boundary_id(&project_id).await;
    let layout_id = app.generate_primary_layout_id(&project_id).await;
    let mut figure = FigurePayload::new(ProjectId(project_id.0));

    let mut layers = Vec::new();
    layers.push(FigureLayerPayload::new(
        FigureLayerDatasourcePayload::SiteBoundary(SiteBoundaryId(boundary_id.0)),
    ));
    layers.push(FigureLayerPayload::new(
        FigureLayerDatasourcePayload::TurbineLayout(TurbineLayoutId(layout_id.0)),
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
        .post_json(&app.api_client, &figure)
        .await;
    assert_ok(&response);
    let figure_id: FigureId = response.json().await.expect("failed to deserialize json");

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
    assert_response_is_pdf(response).await;
    let figure: FigureOutputDTO = app
        .figures_service
        .get_by_id(&app.api_client, &figure_id)
        .await
        .json()
        .await
        .expect("failed to deseriailze figure");
    // Check qgis project is saved
    let qgis_project = app
        .qgis_projects_service
        .get_by_string_id(
            &app.api_client,
            figure.qgis_project_name(&PrintResolution::High).0,
        )
        .await;
    assert_is_qgis_project(qgis_project).await;

    // update the project
    app.figures_service
        .put_json(&app.api_client, &figure_id, &FigurePayload::new(ProjectId(project_id.0)))
        .await;

    // generate new pdf
    let _response = app
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
    let qgis_project = app
        .qgis_projects_service
        .get_by_string_id(
            &app.api_client,
            figure.qgis_project_name(&PrintResolution::High).0,
        )
        .await;

    // assert old qgis project does exist
    assert_is_qgis_project(qgis_project).await
}

#[tokio::test]
async fn get_figure_jpg_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;

    let figure_id = app.generate_figure_id(project_id).await;
    let response = app
        .api_client
        .get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::jpg
        ))
        .send()
        .await
        .expect("failed to execute request");
    assert_ok(&response);
    assert_response_is_jpg(response).await;
    // check the qgis project is in the database
    let figure: FigureOutputDTO = app
        .figures_service
        .get_by_id(&app.api_client, &figure_id)
        .await
        .json()
        .await
        .expect("failed to deseriailze figure");

    let qgis_project = app
        .qgis_projects_service
        .get_by_string_id(
            &app.api_client,
            figure.qgis_project_name(&PrintResolution::Low).0,
        )
        .await;
    assert_is_qgis_project(qgis_project).await;

    // update the project
    app.figures_service
        .put_json(&app.api_client, &figure_id, &FigurePayload::new(ProjectId(project_id.0)))
        .await;

    // generate new jpg
    let _response = app
        .api_client
        .get(format!(
            "{}{}/{}/{}",
            URLS.api.base,
            URLS.api.figures,
            figure_id,
            FigureFormat::jpg
        ))
        .send()
        .await
        .expect("failed to execute request");

    let qgis_project = app
        .qgis_projects_service
        .get_by_string_id(
            &app.api_client,
            figure.qgis_project_name(&PrintResolution::Low).0,
        )
        .await;

    // assert old qgis project does not exist
    assert_eq!(qgis_project.status().as_u16(), 404)
}

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

#[tokio::test]
pub async fn get_figure_with_project_layer_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    let mut figure = FigurePayload::new(ProjectId(project_id.0));
    let mut layers = Vec::new();
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
        .post_json(&app.api_client, &figure)
        .await;
    assert_ok(&response);
    let figure_id: FigureId = response.json().await.expect("failed to deserialize json");
    let response = app
        .figures_service
        .get_by_id(&app.api_client, figure_id)
        .await;
    assert_ok(&response);
}
