use crate::common::{
    TestApp,
    helpers::{assert_status, check_error_response},
};
use app::enums::{Collection, ProjectIdentifier};
use domain::{FeatureId, ProjectId, Slug};
use rstest::rstest;

enum Endpoint {
    GetCollection,
    GetCollections,
    GetFeature,
    GetFeatures,
    GetProject,
    GetProjects,
    GetProjectLandingPage,
    GetProjectConformanceDeclaration,
    GetProjectCollections,
}

impl Endpoint {
    async fn call(&self, app: &TestApp) -> reqwest::Response {
        match self {
            Endpoint::GetCollection => {
                app.ogc_service
                    .get_collection(&app.api_client, &Slug::default())
                    .await
            }
            Endpoint::GetCollections => app.ogc_service.get_collections(&app.api_client).await,
            Endpoint::GetFeatures => {
                app.ogc_service
                    .get_features(&app.api_client, &Slug::default(), None)
                    .await
            }
            Endpoint::GetFeature => {
                app.ogc_service
                    .get_feature(&app.api_client, &Slug::default(), FeatureId::default())
                    .await
            }
            Endpoint::GetProject => {
                app.ogc_service
                    .get_feature(
                        &app.api_client,
                        &Collection::Projects.try_into().unwrap(),
                        FeatureId::default(),
                    )
                    .await
            }
            Endpoint::GetProjectLandingPage => {
                app.ogc_service
                    .get_project_landing_page(
                        &app.api_client,
                        &ProjectIdentifier::Id(ProjectId::default()),
                    )
                    .await
            }
            Endpoint::GetProjectConformanceDeclaration => {
                app.ogc_service
                    .get_project_conformance_declaration(
                        &app.api_client,
                        &ProjectIdentifier::Id(ProjectId::default()),
                    )
                    .await
            }
            Endpoint::GetProjects => {
                app.ogc_service
                    .get_features(
                        &app.api_client,
                        &Collection::Projects.try_into().unwrap(),
                        None,
                    )
                    .await
            }
            Endpoint::GetProjectCollections => {
                app.ogc_service
                    .get_project_collections(
                        &app.api_client,
                        &ProjectIdentifier::Id(ProjectId::default()),
                    )
                    .await
            }
        }
    }
}

#[rstest]
#[actix_web::test]
async fn handler_returns_500_for_fatal_database_error(
    #[values(
        Endpoint::GetCollection,
        Endpoint::GetCollections,
        Endpoint::GetFeatures,
        Endpoint::GetFeature,
        Endpoint::GetProject,
        Endpoint::GetProjects,
        Endpoint::GetProjectConformanceDeclaration,
        Endpoint::GetProjectLandingPage,
        Endpoint::GetProjectCollections
    )]
    endpoint: Endpoint,
) {
    let app = TestApp::spawn_with_db().await;
    app.drop_app_schema().await;
    let response = endpoint.call(&app).await;
    check_error_response(response, 500, "database error").await
}

#[rstest]
#[actix_web::test]
async fn handler_returns_404_for_not_found(
    #[values(
        Endpoint::GetCollection,
        Endpoint::GetFeatures,
        Endpoint::GetFeature,
        Endpoint::GetProject,
        Endpoint::GetProjectLandingPage,
        Endpoint::GetProjectConformanceDeclaration
    )]
    endpoint: Endpoint,
) {
    let app = TestApp::spawn_with_db().await;
    let response = endpoint.call(&app).await;
    assert_status(&response, 404)
}
