use domain::ApiKey;

use crate::common::{
    AppBuilder,
    helpers::{assert_status, handle_json_response},
};

#[actix_web::test]
async fn revoke_api_key_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let key = app.generate_api_key(&token).await;
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&token))
            .await,
    )
    .await
    .unwrap();
    assert!(keys.iter().any(|k| k.id == key.id));
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&token))
        .await;
    assert_status(&response, 204);
    let keys: Vec<ApiKey> = handle_json_response(
        app.api_keys_service
            .get_all(&app.api_client, Some(&token))
            .await,
    )
    .await
    .unwrap();
    assert!(!keys.iter().any(|k| k.id == key.id));
}

#[actix_web::test]
async fn revoke_api_key_returns_404_when_revoking_another_users_key() {
    let app = AppBuilder::new().build().await;
    println!("user 1 id: {}", app.test_user_clerk_id);
    println!("user 2 id: {}", app.test_user_2_clerk_id);
    let token = app.generate_session_token().await;
    let key = app.generate_api_key(&token).await;
    let user_2_token = app.generate_user_2_session_token().await;
    let response = app
        .api_keys_service
        .revoke(&app.api_client, key.id, Some(&user_2_token))
        .await;
    assert_status(&response, 404);
}
