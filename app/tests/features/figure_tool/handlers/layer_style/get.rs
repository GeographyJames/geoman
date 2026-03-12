use geoman::app::features::figure_tool::dtos::LayerStyleOutputDTO;

use crate::{app::TestApp, helpers::assert_ok};

#[tokio::test]
async fn get_layer_styles_works() {
    let app = TestApp::spawn_and_login().await;
    let response = app.layer_styles_service.get_all(&app.api_client).await;
    assert_ok(&response);
    let styles: Vec<LayerStyleOutputDTO> =
        response.json().await.expect("failed to deserialize json");
    assert!(styles.len() > 0);
}
