use actix_web::{HttpResponse, web};

use crate::handlers::webhooks::ClerkWehbookPayload;

#[tracing::instrument(skip(payload))]
pub async fn clerk_webhook(payload: web::Json<ClerkWehbookPayload>) -> HttpResponse {
    tracing::info!("\n\nrequest received!\n");
    tracing::info!("{}", serde_json::to_string_pretty(&payload).unwrap());
    HttpResponse::Ok().finish()
}
