use actix_web::{HttpResponse, web};

use crate::handlers::webhooks::ClerkWebhookPayload;

#[tracing::instrument(skip(payload))]
pub async fn clerk_webhook(payload: web::Json<ClerkWebhookPayload>) -> HttpResponse {
    tracing::info!("\n\nrequest received!\n");
    tracing::info!("{}", serde_json::to_string_pretty(&payload).unwrap());
    HttpResponse::Ok().finish()
}
