use actix_web::{HttpResponse, delete, web};
use domain::BusinessUnitId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{bu_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_business_unit(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<BusinessUnitId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_business_unit(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
