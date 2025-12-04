use crate::common::AppBuilder;

#[actix_web::test]
async fn generate_api_key_works() {
    let app = AppBuilder::new().build().await;

    let _key = app.generate_api_key(None).await;
}
