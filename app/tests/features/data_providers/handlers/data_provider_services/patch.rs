use app::features::data_providers::handlers::DataProviderServiceUpdatePayload;
use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn patch_data_provider_service_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let provider_id = app.generate_data_provider_id(Some(&auth)).await;
    let service_id = app
        .generate_data_provider_service_id(provider_id, Some(&auth))
        .await;

    let response = app
        .data_provider_services_service
        .patch_json(
            &app.api_client,
            service_id.0,
            Some(&auth),
            &DataProviderServiceUpdatePayload {
                name: Some(uuid::Uuid::new_v4().to_string()),
                base_url: Some("https://updated.example.com/wms".to_string()),
                ..Default::default()
            },
        )
        .await;
    assert_status(&response, 204);
}
