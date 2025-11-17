use crate::common::{TestApp, helpers::check_landing_page_response};
use app::enums::ProjectIdentifier;
use domain::Slug;

#[actix_web::test]
async fn get_project_landing_page_works() {
    let app = TestApp::spawn_with_db().await;
    let (_, _, project_id) = app.generate_ids().await;

    let response = app
        .ogc_service
        .get_project_landing_page(&app.api_client, &ProjectIdentifier::Id(project_id))
        .await;
    check_landing_page_response(response).await;
}

#[actix_web::test]
async fn get_project_landing_page_works_with_slug() {
    let app = TestApp::spawn_with_db().await;
    let team_id = app.generate_team_id().await;
    let user_id = app.generate_user_id(team_id).await;
    let name = uuid::Uuid::new_v4().to_string();
    let slug = Slug::parse(uuid::Uuid::new_v4().to_string()).unwrap();
    let _project_id = app.insert_project(&name, &slug, user_id).await;
    let response = app
        .ogc_service
        .get_project_landing_page(&app.api_client, &ProjectIdentifier::Slug(slug.into_inner()))
        .await;
    check_landing_page_response(response).await;
}
