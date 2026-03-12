use app::features::figure_tool::dtos::LayerStyleOutputDTO;
use domain::TeamId;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_layer_styles_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);

    let response = app
        .layer_styles_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
    let styles: Vec<LayerStyleOutputDTO> = handle_json_response(response)
        .await
        .expect("failed to retrieve layer styles");
    assert!(styles.is_empty());
}
