use app::features::figure_tool::dtos::ProjectLayerOutputDTO;
use serde::Serialize;

use crate::common::{TestApp, helpers::assert_ok};

use super::VALID_TABLE_NAMES;

#[derive(Serialize)]
struct ProjectLayerParams {
    project: i32,
}

#[tokio::test]
async fn get_project_layers_works() {
    let (app, user, project_id) = TestApp::with_project().await;

    // Seed project_data tables. The SQL file uses p0001_* prefix, so project_id must be 1
    // (guaranteed in a fresh test DB where this is the first project created).
    app.execute_sql_file("../seed_data/project_data_tables.sql").await;

    let response = app
        .project_layers_service
        .get_with_params(
            &app.api_client,
            Some(&user),
            &ProjectLayerParams { project: project_id.0 },
        )
        .await;
    assert_ok(&response);

    let mut layers: Vec<ProjectLayerOutputDTO> =
        response.json().await.expect("failed to deserialize json");
    layers.sort_by(|a, b| a.table_name.cmp(&b.table_name));

    let mut expected: Vec<&str> = VALID_TABLE_NAMES.to_vec();
    expected.sort();

    let actual: Vec<&str> = layers.iter().map(|l| l.table_name.as_str()).collect();
    assert_eq!(actual, expected);

    // Project 24 has a single matching table
    let layers: Vec<ProjectLayerOutputDTO> = app
        .project_layers_service
        .get_with_params(
            &app.api_client,
            Some(&user),
            &ProjectLayerParams { project: 24 },
        )
        .await
        .json()
        .await
        .expect("failed to deserialize json");

    let actual: Vec<&str> = layers.iter().map(|l| l.table_name.as_str()).collect();
    assert_eq!(actual, ["p0024 3 test polygons"]);
}
