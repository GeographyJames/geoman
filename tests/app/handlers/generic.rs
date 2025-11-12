use crate::app::{
    TestApp,
    helpers::{assert_status, check_error_response},
};
use geoman::{
    constants::DB_QUERY_FAIL,
    domain::{FeatureId, Slug},
};

#[actix_web::test]
async fn handler_returns_500_for_fatal_databas_error() {
    let app = TestApp::spawn_with_db().await;
    app.drop_app_schema().await;
    let responses = [
        app.ogc_service
            .get_collection(&app.api_client, &Slug::default())
            .await,
        app.ogc_service.get_collections(&app.api_client).await,
        app.ogc_service
            .get_features(&app.api_client, &Slug::default(), None)
            .await,
        app.ogc_service
            .get_feature(&app.api_client, &Slug::default(), FeatureId::default())
            .await,
    ];
    for response in responses {
        check_error_response(response, 500, DB_QUERY_FAIL).await
    }
}

#[actix_web::test]
async fn handler_returns_404_for_non_existent_data() {
    let app = TestApp::spawn_with_db().await;
    let responses = [
        app.ogc_service
            .get_collection(&app.api_client, &Slug::default())
            .await,
        app.ogc_service
            .get_features(&app.api_client, &Slug::default(), None)
            .await,
        app.ogc_service
            .get_feature(&app.api_client, &Slug::default(), FeatureId::default())
            .await,
    ];
    for response in responses {
        assert_status(&response, 404)
    }
}
