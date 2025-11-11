use std::sync::LazyLock;

use actix_web::web;
use utoipa::{OpenApi, openapi::OpenApi as ApiDocs};

use crate::app::ApiDoc;

static API_DOC: LazyLock<ApiDocs> = LazyLock::new(ApiDoc::openapi);

#[tracing::instrument]
pub async fn get_api_docs() -> web::Json<&'static ApiDocs> {
    web::Json(&API_DOC)
}
