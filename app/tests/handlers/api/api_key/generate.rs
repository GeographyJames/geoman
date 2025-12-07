use crate::common::{AppBuilder, Auth};

#[actix_web::test]
async fn generate_api_key_works() {
    let app = AppBuilder::new().build().await;

    let _key = app.generate_api_key(Some(&Auth::mock_session_token())).await;
}
