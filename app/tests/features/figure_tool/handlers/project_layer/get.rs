use geoman::app::features::figure_tool::{dtos::ProjectLayerOutputDTO, ids::ProjectId};

use crate::{app::TestApp, helpers::assert_ok};

#[tokio::test]
async fn get_project_layers_works() {
    let mut expected_tables: Vec<&str> = VALID_TABLE_NAMES.to_vec();
    expected_tables.sort();

    let app = TestApp::spawn_and_login().await;
    let response = app
        .project_layers_service
        .get_all_for_project(&app.api_client, ProjectId(1))
        .await;
    assert_ok(&response);
    let layers: Vec<ProjectLayerOutputDTO> =
        response.json().await.expect("failed to deserialize json");

    let mut actual_tables: Vec<&str> = layers
        .iter()
        .map(|layer| layer.table_name.as_str())
        .collect();
    actual_tables.sort();

    assert_eq!(actual_tables, expected_tables);

    let layers: Vec<ProjectLayerOutputDTO> = app
        .project_layers_service
        .get_all_for_project(&app.api_client, ProjectId(24))
        .await
        .json()
        .await
        .unwrap();

    let actual_tables: Vec<&str> = layers
        .iter()
        .map(|layer| layer.table_name.as_str())
        .collect();

    assert_eq!(actual_tables, ["p0024 3 test polygons"]);
}
