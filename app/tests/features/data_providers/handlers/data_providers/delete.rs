use domain::TeamId;

use crate::common::{AppBuilder, Auth, helpers::assert_status};

#[tokio::test]
async fn delete_data_provider_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let id = app.generate_data_provider_id(Some(&auth)).await;

    let response = app
        .data_providers_service
        .delete(&app.api_client, id.0, Some(&auth))
        .await;
    assert_status(&response, 204);
}
