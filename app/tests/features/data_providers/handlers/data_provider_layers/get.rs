use app::features::data_providers::DataProviderLayer;
use domain::TeamId;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_data_provider_layers_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let provider_id = app.generate_data_provider_id(Some(&auth)).await;
    let service_id = app
        .generate_data_provider_service_id(provider_id, Some(&auth))
        .await;
    let _id = app
        .generate_data_provider_layer_id(service_id, Some(&auth))
        .await;

    let response = app
        .data_provider_layers_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
    let layers: Vec<DataProviderLayer> = handle_json_response(response)
        .await
        .expect("failed to retrieve data provider layers");
    assert_eq!(layers.len(), 1);
}
