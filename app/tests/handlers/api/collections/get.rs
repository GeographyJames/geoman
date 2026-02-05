use crate::common::{AppBuilder, Auth, helpers::assert_ok};

#[actix_web::test]
pub async fn get_collections_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let _collection_id = app.generate_project_collection_id(Some(&auth)).await;
    let response = app
        .collections_service
        .get(&app.api_client, Some(&auth))
        .await;
    assert_ok(&response);
}
