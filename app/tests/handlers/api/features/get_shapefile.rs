use app::constants::SITE_BOUNDARIES_COLLECTION_ID;

use crate::common::{
    AppBuilder, Auth,
    helpers::{assert_ok, handle_json_response},
};
use domain::enums::CollectionId;
use gdal::{Dataset, vector::LayerAccess};
use geo::virtual_shapefile::VirtualFile;

#[actix_web::test]
async fn get_shapefile_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let boundary_id = app
        .generate_primary_boundary_id(project_id, Some(&auth))
        .await;
    let collection = app
        .ogc_service
        .get_project_collection_ogc(
            &app.api_client,
            project_id,
            domain::ProjectCollectionId(SITE_BOUNDARIES_COLLECTION_ID),
        )
        .await;
    let project: ogc::features::Feature = handle_json_response(
        app.ogc_service
            .get_feature(
                &app.api_client,
                &CollectionId::Projects.to_string(),
                project_id.0,
            )
            .await,
    )
    .await
    .expect("failed to retrieve project");
    let project_slug = match project.properties.get("slug").expect("no project slug") {
        serde_json::Value::String(slug) => slug,
        _ => panic!("project slug not a string"),
    };

    let collection_slug = match collection
        .additional_properties
        .get("slug")
        .expect("no collection slug")
    {
        serde_json::Value::String(slug) => slug,
        _ => panic!("collection slug not a string"),
    };
    let response = app
        .get_feature_shapefile(
            Some(&auth),
            project_slug,
            collection_slug,
            boundary_id.feature_id.0,
        )
        .await;
    assert_ok(&response);
    let body = response.bytes().await.expect("no resposne body");
    let virtual_file_filename = format!("{}.shz", uuid::Uuid::new_v4());
    let _virtual_file = VirtualFile::new(&virtual_file_filename, body.to_vec())
        .expect("failed to create virtual file");
    let datasest = Dataset::open(format!("/vsimem/{}", virtual_file_filename))
        .expect("failed to open virtual file");
    let mut layer = datasest.layers().next().expect("no layer");
    let srs = layer.spatial_ref().expect("layer has no spatial ref");
    let auth_code = srs.auth_code().expect("failed to get srs auth code");
    assert_eq!(auth_code, 27700);
    let _geom = layer
        .features()
        .next()
        .expect("no features")
        .geometry()
        .expect("no geometry");
}
