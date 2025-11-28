use crate::common::AppBuilder;

#[actix_web::test]
async fn generate_api_key_works() {
    let app = AppBuilder::new().build().await;
    let token = app.generate_session_token().await;
    let _key = app.generate_api_key(&token).await;
}
