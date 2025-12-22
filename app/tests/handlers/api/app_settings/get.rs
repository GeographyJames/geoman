use crate::common::AppBuilder;

#[tokio::test]
async fn get_app_settings_works() {
    let app = AppBuilder::new().build().await;
    let _settings = app.get_app_settings().await;
}
