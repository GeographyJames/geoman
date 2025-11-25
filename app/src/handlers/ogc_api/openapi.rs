use crate::{AppState, handlers::ApiError};
use actix_web::{HttpRequest, get, web};
use anyhow::anyhow;
use utoipa::openapi::{self, Server};

#[get("")]
#[tracing::instrument(skip(state, req))]
pub async fn get_openapi(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<web::Json<openapi::OpenApi>, ApiError> {
    let (parent_path, _) = req
        .path()
        .rsplit_once("/")
        .ok_or_else(|| ApiError::Unexpected(anyhow!("Failed to retrieve server path")))?;
    let mut openapi = state.openapi.to_owned();
    openapi.servers = Some(vec![Server::new(parent_path)]);
    Ok(web::Json(openapi))
}
