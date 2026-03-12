use geoman::app::features::figure_tool::dtos::base_map::BaseMapOutputDTO;

use crate::{app::TestApp, helpers::assert_ok};

#[tokio::test]
async fn get_base_maps_works() {
    let app = TestApp::spawn_and_login().await;
    let response = app.base_maps_service.get_all(&app.api_client).await;
    assert_ok(&response);
    let base_maps: Vec<BaseMapOutputDTO> =
        response.json().await.expect("failed to deserialize json");
    assert!(!base_maps.is_empty())
}
