use domain::TeamId;

use crate::common::AppBuilder;

#[actix_web::test]
async fn delete_collection_works() {
    let app = AppBuilder::new().build().await;
    let auth = app._generate_user(false, TeamId(0)).await;
    let collection_id = app
        .generate_project_collection_id(Some(&crate::common::Auth::MockUserCredentials(auth)))
        .await;
}
