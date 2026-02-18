use gdal::{Dataset, vector::LayerAccess};
use geo::virtual_shapefile::VirtualFile;

use crate::common::{AppBuilder, Auth, helpers::assert_ok};

#[actix_web::test]
async fn get_shapefile_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::mock_session_token();
    let project_id = app.generate_project_id(Some(&auth)).await;
    let boundary_id = app
        .generate_primary_boundary_id(project_id, Some(&auth))
        .await;
    let response = app
        .features_service
        .get_one(&app.api_client, Some(&auth), boundary_id.feature_id)
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
