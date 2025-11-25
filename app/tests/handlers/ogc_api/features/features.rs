use domain::TableName;
use serde::{Deserialize, Serialize};

use crate::common::{
    TestApp,
    helpers::{assert_ok, generate_random_wgs84_point_ewkt, handle_json_response},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Properties {
    some_text: String,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            some_text: uuid::Uuid::new_v4().to_string(),
        }
    }
}

async fn test_setup(input_text: &str) -> (TestApp, TableName) {
    let app = TestApp::spawn_with_db().await;

    let table_name = app.generate_gis_data_table_name().await;
    for _ in 0..10 {
        let (_, _, ewkt) = generate_random_wgs84_point_ewkt();
        let _feature = app.insert_feature(&table_name, &ewkt, input_text).await;
    }

    (app, table_name)
}

#[actix_web::test]
async fn get_features_works() {
    let text = uuid::Uuid::new_v4().to_string();
    let (app, table_name) = test_setup(&text).await;

    let response = app
        .ogc_service
        .get_features(&app.api_client, table_name.as_ref())
        .await;

    assert_ok(&response);

    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("failed to retrieve feature collection");

    assert_eq!(feature_collection.features.len(), 10);
    assert!(feature_collection.number_matched == 10);
    assert!(feature_collection.number_returned == 10);
    let ogc_feature = feature_collection.features.into_iter().next().unwrap();
    let props: Properties =
        serde_json::from_value(serde_json::Value::Object(ogc_feature.properties))
            .expect("failed to deserialise json");
    assert!(props.some_text == text)
}

#[actix_web::test]
async fn get_features_works_with_limit() {
    let text = uuid::Uuid::new_v4().to_string();
    let (app, table) = test_setup(&text).await;

    let limit = 5;

    let response = app
        .ogc_service
        .get_features_with_params(&app.api_client, table.as_ref(), &[("limit", limit)])
        .await;
    assert_ok(&response);
    let feature_collection: ogc::FeatureCollection = handle_json_response(response)
        .await
        .expect("Failed to retrieve feature collection");
    assert_eq!(feature_collection.features.len(), limit);
}
