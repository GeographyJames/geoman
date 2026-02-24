use actix_web::{HttpResponse, patch, web};
use domain::BusinessUnitId;
use serde::{Deserialize, Serialize};

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[derive(Serialize, Deserialize)]
pub struct BusinessUnitUpdatePayload {
    pub name: Option<String>,
}

#[patch("/{bu_id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_business_unit(
    repo: web::Data<PostgresRepo>,
    body: web::Json<BusinessUnitUpdatePayload>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<BusinessUnitId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    let user = user.into_inner();
    repo.update(&(body.into_inner(), id.into_inner(), user.id))
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
