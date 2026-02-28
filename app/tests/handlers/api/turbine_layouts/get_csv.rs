use app::constants::TURBINE_LAYOUTS_COLLECTION_ID;
use domain::{ProjectCollectionId, TeamId};

use crate::common::{AppBuilder, Auth, helpers::assert_ok};

#[tokio::test]
async fn get_turbine_layout_csv_works() {
    let app = AppBuilder::new().build().await;
    let user = Auth::MockUserCredentials(app.generate_user(false, TeamId(0)).await);
    let project_id = app.generate_project_id(Some(&user)).await;
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
        .get_feature_csv(
            Some(&user),
            project_slug.as_ref(),
            collection_slug.as_ref(),
            layout_id.0,
        )
        .await;
    assert_ok(&response);
    let body = response.bytes().await.expect("response has no body");
    assert!(!body.is_empty());
    match String::from_utf8(body.into()) {
        Ok(s) => {
            let first_line = s.split("\n").next().expect("no first line");
            assert_eq!(
                first_line,
                "id,turbine_number,hub_height_m,blade_length_m,latitude,longitude,x_27700,y_27700"
            )
        }
        Err(e) => panic!("failed to read bytes to string: {}", e),
    }
}
