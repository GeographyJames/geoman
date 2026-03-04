use domain::{DataProvider, TeamId};

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};

#[tokio::test]
async fn get_data_providers_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);
    let _id = app.generate_data_provider_id(Some(&auth)).await;

    let response = app
        .data_providers_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
    let providers: Vec<DataProvider> = handle_json_response(response)
        .await
        .expect("failed to retrieve data providers");
    assert_eq!(providers.len(), 1);
}
