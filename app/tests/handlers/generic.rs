use crate::common::{
    TestApp,
    helpers::{assert_status, check_error_response},
};
use domain::{FeatureId, Slug};
use rstest::rstest;

enum Endpoint {
    GetCollection,
    GetCollections,
    GetFeature,
    GetFeatures,
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
        }
    }
}

#[rstest]
#[actix_web::test]
async fn handler_returns_500_for_fatal_databas_error(
    #[values(
        Endpoint::GetCollection,
        Endpoint::GetCollections,
        Endpoint::GetFeatures,
        Endpoint::GetFeature
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
    #[values(Endpoint::GetCollection, Endpoint::GetFeatures, Endpoint::GetFeature)]
    endpoint: Endpoint,
) {
    let app = TestApp::spawn_with_db().await;
    let response = endpoint.call(&app).await;
    assert_status(&response, 404)
}
