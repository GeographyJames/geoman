use actix_web::{HttpResponse, patch, web};
use domain::{ProjectId, project::ProjectUpdateDto};

use crate::{
    AuthenticatedUser, errors::ApiError, handlers::api::projects::PatchProjectPayload,
    postgres::PostgresRepo,
};

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_project(
    id: web::Path<ProjectId>,
    body: web::Json<PatchProjectPayload>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    let dto: ProjectUpdateDto = body.into_inner().try_into_dto(id.into_inner())?;
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::Ok().finish())
}
