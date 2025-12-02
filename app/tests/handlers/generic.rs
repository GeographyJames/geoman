use crate::common::{
    TestApp,
    helpers::{assert_status, check_error_response},
    services::AuthService,
};
use domain::{ProjectCollectionId, ProjectId, enums::CollectionId};
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
    GetProjectCollection,
    GetProjectFeatures,
    GetProjectFeature,
}

impl Endpoint {
    async fn call<T: AuthService>(&self, app: &TestApp<T>) -> reqwest::Response {
        match self {
            Endpoint::GetCollection => {
                app.ogc_service
                    .get_collection(&app.api_client, "a-table")
                    .await
            }
            Endpoint::GetCollections => app.ogc_service.get_collections(&app.api_client).await,
            Endpoint::GetFeatures => {
                app.ogc_service
                    .get_features(&app.api_client, "a-table")
                    .await
            }
            Endpoint::GetFeature => {
                app.ogc_service
                    .get_feature(&app.api_client, "a-table", 0)
                    .await
            }
            Endpoint::GetProject => {
                app.ogc_service
                    .get_feature(&app.api_client, &CollectionId::Projects.to_string(), 0)
                    .await
            }
            Endpoint::GetProjectLandingPage => {
                app.ogc_service
                    .get_project_landing_page(&app.api_client, ProjectId::default())
                    .await
            }
            Endpoint::GetProjectConformanceDeclaration => {
                app.ogc_service
                    .get_project_conformance_declaration(&app.api_client, ProjectId::default())
                    .await
            }
            Endpoint::GetProjects => {
                app.ogc_service
                    .get_features(&app.api_client, &CollectionId::Projects.to_string())
                    .await
            }
            Endpoint::GetProjectCollections => {
                app.ogc_service
                    .get_project_collections(&app.api_client, ProjectId::default())
                    .await
            }
            Endpoint::GetProjectFeatures => {
                app.ogc_service
                    .get_project_features(
                        &app.api_client,
                        ProjectCollectionId::default(),
                        ProjectId::default(),
                    )
                    .await
            }
            Endpoint::GetProjectCollection => {
                app.ogc_service
                    .get_project_collection(
                        &app.api_client,
                        ProjectId::default(),
                        ProjectCollectionId::default(),
                    )
                    .await
            }
            Endpoint::GetProjectFeature => {
                app.ogc_service
                    .get_project_feature(
                        &app.api_client,
                        ProjectId::default(),
                        ProjectCollectionId::default(),
                        0,
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
        Endpoint::GetProjectCollections,
        Endpoint::GetProjectCollection,
        Endpoint::GetProjectFeatures,
        Endpoint::GetProjectFeature
    )]
    endpoint: Endpoint,
) {
    let app = TestApp::spawn_with_db().await;
    app.drop_database().await;
    let response = endpoint.call(&app).await;
    let err = check_error_response(response, 500).await;
    assert_eq!(err.message, "database error");
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
        Endpoint::GetProjectConformanceDeclaration,
        Endpoint::GetProjectCollections,
        Endpoint::GetProjectCollection,
        Endpoint::GetProjectFeatures,
        Endpoint::GetProjectFeature
    )]
    endpoint: Endpoint,
) {
    let app = TestApp::spawn_with_db().await;
    let response = endpoint.call(&app).await;
    assert_status(&response, 404)
}
