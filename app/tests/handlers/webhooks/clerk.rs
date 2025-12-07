use app::{URLS, handlers::webhooks::ClerkWebhookPayload};

use crate::common::{AppBuilder, constants::REQUEST_FAILED, helpers::assert_ok};

#[actix_web::test]
async fn clerk_webhook_works() {
    let app = AppBuilder::new().build().await;
    let body = ClerkWebhookPayload {
        event_type: "todo".to_string(),
        data: Default::default(),
    };
    let response = app
        .api_client
        .post(format!("{}{}", URLS.webhooks.base, URLS.webhooks.clerk))
        .json(&body)
        .send()
        .await
        .expect(REQUEST_FAILED);
    assert_ok(&response)
}
