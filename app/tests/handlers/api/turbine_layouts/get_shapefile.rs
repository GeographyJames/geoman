use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::ProjectCollectionId;
use gdal::{Dataset, vector::LayerAccess};
use geo::virtual_shapefile::VirtualFile;

use crate::common::{TestApp, helpers::assert_ok};

#[tokio::test]
async fn get_turbine_layout_shapefile_works() {
    let (app, user, project_id) = TestApp::with_project().await;
    let layout_id = app
        .generate_primary_layout_id(&project_id, Some(&user))
        .await;
    let collection_slug = app
        .get_collection_slug(
            project_id,
            ProjectCollectionId(TURBINE_LAYOUTS_COLLECTION_ID),
        )
        .await;

    let project_slug = app.get_project_slug(project_id).await;
    let response = app
        .get_feature_shapefile(
            Some(&user),
            project_slug.as_ref(),
            collection_slug.as_ref(),
            layout_id.0,
        )
        .await;
    assert_ok(&response);
    let body = response.bytes().await.expect("no response body");
    let filename = format!("{}.shz", uuid::Uuid::new_v4());
    let _virtual_file =
        VirtualFile::new(&filename, body.to_vec()).expect("failed to create virtual file");
    let ds = Dataset::open(format!("/vsimem/{}", filename)).expect("failed to open virtual file");
    let mut layer = ds.layers().next().expect("no layer");
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
