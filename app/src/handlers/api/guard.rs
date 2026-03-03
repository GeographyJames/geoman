use domain::ProjectId;
use sqlx::PgPool;

use crate::{
    AuthenticatedUser, constants::UNASSIGNED_USERS_TEAM_ID, errors::ApiError, repo::RepositoryError,
};

pub struct ProjectWriteAccess {
    pub is_owner: bool,
}

pub async fn check_project_write_access(
    pool: &PgPool,
    project_id: ProjectId,
    user: &AuthenticatedUser,
    action: &str,
) -> Result<ProjectWriteAccess, ApiError> {
    if user.team_id.0 == UNASSIGNED_USERS_TEAM_ID {
        return Err(ApiError::UnassignedUser);
    }
    let res = sqlx::query!(
        "SELECT p.owner AS owner_id,
                u.team_id AS project_owner_team_id
           FROM app.projects p
           JOIN app.users u ON p.owner = u.id
          WHERE p.id = $1",
        project_id.0
    )
    .fetch_one(pool)
    .await
    .map_err(RepositoryError::from)?;

    let same_team = res.project_owner_team_id == user.team_id.0;

    if !(user.admin || same_team) {
        return Err(ApiError::Forbidden(format!(
            "User does not have permission to {}",
            action
        )));
    }

    Ok(ProjectWriteAccess {
        is_owner: res.owner_id == user.id.0,
    })
}
