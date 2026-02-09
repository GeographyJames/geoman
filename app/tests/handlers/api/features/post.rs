use app::handlers::api::project_collections::CollectionReqPayload;
use domain::{ProjectCollectionId, enums::GeometryType};

use crate::common::{AppBuilder, Auth, helpers::handle_json_response};

#[actix_web::test]
async fn post_shapefile_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let collection = CollectionReqPayload {
        title: uuid::Uuid::new_v4().to_string(),
        geometry_type: GeometryType::MultiPolygon,
        description: None,
    };
    let collection_id: ProjectCollectionId = handle_json_response(
        app.collections_service
            .post_json(&app.api_client, Some(&auth), &collection)
            .await,
    )
    .await
    .unwrap();
}
