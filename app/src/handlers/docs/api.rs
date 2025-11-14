use crate::ApiDoc;
use actix_web::web;
use std::sync::LazyLock;
use utoipa::{OpenApi, openapi::OpenApi as ApiDocs};

static API_DOC: LazyLock<ApiDocs> = LazyLock::new(ApiDoc::openapi);

#[tracing::instrument]
pub async fn get_api_docs() -> web::Json<&'static ApiDocs> {
    web::Json(&API_DOC)
}
