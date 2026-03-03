use actix_web::{HttpResponse, patch, web};
use domain::{ProjectId, enums::Status, project::ProjectUpdateDto};

use crate::{
    AuthenticatedUser,
    errors::ApiError,
    handlers::api::{guard::check_project_write_access, projects::PatchProjectPayload},
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
    let access = check_project_write_access(&repo.db_pool, *id, &user, "alter project").await?;

    if let Some(ref status) = body.status
        && status == &Status::Deleted
        && !(user.admin || access.is_owner)
    {
        return Err(ApiError::Forbidden(
            "User does not have permission to delete project".to_string(),
        ));
    };

    let dto: ProjectUpdateDto = body.into_inner().try_into_dto(id.into_inner())?;
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::NoContent().finish())
}
