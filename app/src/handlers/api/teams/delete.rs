use actix_web::{HttpResponse, delete, web};
use domain::TeamId;

use crate::{AuthenticatedUser, errors::ApiError, postgres::PostgresRepo};

#[delete("/{team_id}")]
#[tracing::instrument(skip(repo, user, id))]
pub async fn delete_team(
    repo: web::Data<PostgresRepo>,
    user: web::ReqData<AuthenticatedUser>,
    id: web::Path<TeamId>,
) -> Result<HttpResponse, ApiError> {
    if !user.admin {
        return Err(ApiError::AdminOnly);
    }
    repo.delete_team(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::{
        AuthenticatedUser, MockUserCredentials, testing::test_helpers::mock_app_with_path_params,
    };

    use super::*;

    #[actix_web::test]
    async fn delete_team_requires_admin_permission() {
        let req = test::TestRequest::delete().uri("/1");
        let resp = mock_app_with_path_params(
            delete_team,
            req,
            MockUserCredentials::User(AuthenticatedUser::default()),
        )
        .await;
        assert_eq!(resp.status(), 401);
    }
}
