use domain::TeamId;

use crate::common::{AppBuilder, Auth};

#[tokio::test]
async fn post_data_provider_layer_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let provider_id = app.generate_data_provider_id(Some(&auth)).await;
    let service_id = app
        .generate_data_provider_service_id(provider_id, Some(&auth))
        .await;

    let _id = app
        .generate_data_provider_layer_id(service_id, Some(&auth))
        .await;
}
