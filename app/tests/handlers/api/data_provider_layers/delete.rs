use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn delete_data_provider_layer_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let provider_id = app.generate_data_provider_id(Some(&auth)).await;
    let service_id = app
        .generate_data_provider_service_id(provider_id, Some(&auth))
        .await;
    let layer_id = app
        .generate_data_provider_layer_id(service_id, Some(&auth))
        .await;

    let response = app
        .data_provider_layers_service
        .delete(&app.api_client, layer_id.0, Some(&auth))
        .await;
    assert_status(&response, 204);
}
