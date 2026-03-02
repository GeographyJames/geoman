use actix_web::{HttpResponse, patch, web};
use domain::{ProjectId, enums::Status, project::ProjectUpdateDto};

use crate::{
    AuthenticatedUser, constants::UNASSIGNED_USERS_TEAM_ID, errors::ApiError,
    handlers::api::projects::PatchProjectPayload, postgres::PostgresRepo, repo::RepositoryError,
};

#[patch("/{id}")]
#[tracing::instrument(skip(repo, body, user, id))]
pub async fn patch_project(
    id: web::Path<ProjectId>,
    body: web::Json<PatchProjectPayload>,
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, ApiError> {
    if user.team_id.0 == UNASSIGNED_USERS_TEAM_ID {
        return Err(ApiError::UnassignedUser);
    }
    let res = sqlx::query!(
        "SELECT p.owner AS owner_id,
                u.team_id AS project_owner_team_id
           FROM app.projects p
           JOIN app.users u ON p.owner = u.id
          WHERE p.id = $1",
        id.as_ref().0
    )
    .fetch_one(&repo.db_pool)
    .await
    .map_err(RepositoryError::from)?;

    let is_admin = user.admin;
    let is_owner = res.owner_id == user.id.0;
    let same_team = res.project_owner_team_id == user.team_id.0;

    if let Some(ref status) = body.status
        && status == &Status::Deleted
            && !(user.admin || is_owner) {
                return Err(ApiError::Forbidden(
                    "User does not have permission to delete project".to_string(),
                ));
            };

    if !(is_admin || same_team) {
        return Err(ApiError::Forbidden(
            "User does not have permission to edit project".to_string(),
        ));
    }
    let dto: ProjectUpdateDto = body.into_inner().try_into_dto(id.into_inner())?;
    repo.update(&(&dto, user.id)).await?;
    Ok(HttpResponse::NoContent().finish())
}
