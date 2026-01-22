use serde::{Deserialize, Serialize};

use crate::common::{
    TestApp,
    helpers::{check_error_response, generate_random_wgs84_point_ewkt, handle_json_response},
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

#[actix_web::test]
async fn get_feature_returns_404_for_non_existent_feature() {
    let app = TestApp::spawn_with_db().await;
    let table = app.generate_gis_data_table_name().await;
    let response = app
        .ogc_service
        .get_feature(&app.api_client, table.as_ref(), 0)
        .await;
    check_error_response(response, 404).await;
}

#[actix_web::test]
async fn get_feature_works() {
    let app = TestApp::spawn_with_db().await;
    let table = app.generate_gis_data_table_name().await;
    let text = uuid::Uuid::new_v4().to_string();
    let (_, _, ewkt) = generate_random_wgs84_point_ewkt();
    let feature = app.insert_feature(&table, &ewkt, &text).await;

    let response = app
        .ogc_service
        .get_feature(&app.api_client, table.as_ref(), feature)
        .await;
    let ogc_feature: ogc::features::Feature = handle_json_response(response)
        .await
        .expect("failed to retrieve feature");
    let props: Properties =
        serde_json::from_value(serde_json::Value::Object(ogc_feature.properties))
            .expect("failed to deserialise json");
    assert!(props.some_text == text)
}
