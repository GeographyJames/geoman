use domain::{DataProviderService, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_data_provider_services_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let provider_id = app.generate_data_provider_id(Some(&auth)).await;
    let _id = app
        .generate_data_provider_service_id(provider_id, Some(&auth))
        .await;

    let response = app
        .data_provider_services_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
    let services: Vec<DataProviderService> = handle_json_response(response)
        .await
        .expect("failed to retrieve data provider services");
    assert_eq!(services.len(), 1);
}
